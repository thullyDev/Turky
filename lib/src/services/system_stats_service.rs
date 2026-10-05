use std::time::{Duration, Instant};

use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::Nvml;
use winapi::shared::minwindef::FILETIME;
use winapi::um::processthreadsapi::GetSystemTimes;
use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

const SAMPLE_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, PartialEq)]
pub struct SystemStats {
    pub cpu_percent: f32,
    pub ram_used_bytes: u64,
    pub ram_total_bytes: u64,
    pub gpu_percent: Option<f32>,
    pub gpu_memory_used_bytes: Option<u64>,
    pub gpu_memory_total_bytes: Option<u64>,
    pub cpu_temp_celsius: Option<f32>,
    pub gpu_temp_celsius: Option<f32>,
}

impl SystemStats {
    pub fn empty() -> Self {
        Self {
            cpu_percent: 0.0,
            ram_used_bytes: 0,
            ram_total_bytes: 0,
            gpu_percent: None,
            gpu_memory_used_bytes: None,
            gpu_memory_total_bytes: None,
            cpu_temp_celsius: None,
            gpu_temp_celsius: None,
        }
    }
}

pub struct SystemStatsService {
    reader: Box<dyn StatsReader>,
}

impl SystemStatsService {
    pub fn new() -> Self {
        Self {
            reader: Box::new(LazyHostReader { inner: None }),
        }
    }

    pub fn collect(&mut self) -> SystemStats {
        self.reader.read()
    }

    #[cfg(test)]
    pub fn fixed(stats: SystemStats) -> Self {
        Self {
            reader: Box::new(FixedReader { stats }),
        }
    }
}

trait StatsReader: Send {
    fn read(&mut self) -> SystemStats;
}

#[cfg(test)]
struct FixedReader {
    stats: SystemStats,
}

#[cfg(test)]
impl StatsReader for FixedReader {
    fn read(&mut self) -> SystemStats {
        self.stats.clone()
    }
}

struct LazyHostReader {
    inner: Option<HostReader>,
}

impl StatsReader for LazyHostReader {
    fn read(&mut self) -> SystemStats {
        if self.inner.is_none() {
            self.inner = Some(HostReader::new());
        }

        self.inner.as_mut().expect("reader was just created").read()
    }
}

struct CpuTimes {
    idle: u64,
    total: u64,
}

struct HostReader {
    previous_cpu: Option<CpuTimes>,
    last_sample: Instant,
    cached: SystemStats,
    nvml: Option<Nvml>,
    gpu_unavailable: bool,
}

struct GpuSample {
    percent: Option<f32>,
    used_bytes: Option<u64>,
    total_bytes: Option<u64>,
    temp_celsius: Option<f32>,
}

impl HostReader {
    fn new() -> Self {
        let mut reader = Self {
            previous_cpu: None,
            last_sample: Instant::now() - SAMPLE_INTERVAL,
            cached: SystemStats::empty(),
            nvml: None,
            gpu_unavailable: false,
        };

        reader.sample();
        std::thread::sleep(Duration::from_millis(200));
        reader.sample();

        reader
    }

    fn read(&mut self) -> SystemStats {
        if self.last_sample.elapsed() >= SAMPLE_INTERVAL {
            self.sample();
        }

        self.cached.clone()
    }

    fn sample(&mut self) {
        let current = cpu_times();
        let cpu_percent = match (&self.previous_cpu, &current) {
            (Some(previous), Some(current)) => cpu_percent(previous, current),
            _ => 0.0,
        };

        self.previous_cpu = current;

        let (ram_used_bytes, ram_total_bytes) = memory_bytes();
        let gpu = self.read_gpu();

        self.cached = SystemStats {
            cpu_percent,
            ram_used_bytes,
            ram_total_bytes,
            gpu_percent: gpu.percent,
            gpu_memory_used_bytes: gpu.used_bytes,
            gpu_memory_total_bytes: gpu.total_bytes,
            cpu_temp_celsius: None,
            gpu_temp_celsius: gpu.temp_celsius,
        };

        self.last_sample = Instant::now();
    }

    fn read_gpu(&mut self) -> GpuSample {
        if self.gpu_unavailable {
            return GpuSample::empty();
        }

        if self.nvml.is_none() {
            match Nvml::init() {
                Ok(nvml) => self.nvml = Some(nvml),
                Err(_) => {
                    self.gpu_unavailable = true;

                    return GpuSample::empty();
                }
            }
        }

        let Some(nvml) = self.nvml.as_ref() else {
            return GpuSample::empty();
        };

        let Ok(device) = nvml.device_by_index(0) else {
            return GpuSample::empty();
        };

        let percent = device
            .utilization_rates()
            .ok()
            .map(|rates| rates.gpu as f32);

        let memory = device.memory_info().ok();

        let temp_celsius = device
            .temperature(TemperatureSensor::Gpu)
            .ok()
            .map(|value| value as f32);

        GpuSample {
            percent,
            used_bytes: memory.as_ref().map(|info| info.used),
            total_bytes: memory.as_ref().map(|info| info.total),
            temp_celsius,
        }
    }
}

impl GpuSample {
    fn empty() -> Self {
        Self {
            percent: None,
            used_bytes: None,
            total_bytes: None,
            temp_celsius: None,
        }
    }
}

fn memory_bytes() -> (u64, u64) {
    unsafe {
        let mut status: MEMORYSTATUSEX = std::mem::zeroed();

        status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

        if GlobalMemoryStatusEx(&mut status) == 0 {
            return (0, 0);
        }

        let total = status.ullTotalPhys;
        let used = total.saturating_sub(status.ullAvailPhys);

        (used, total)
    }
}

fn cpu_times() -> Option<CpuTimes> {
    unsafe {
        let mut idle: FILETIME = std::mem::zeroed();
        let mut kernel: FILETIME = std::mem::zeroed();
        let mut user: FILETIME = std::mem::zeroed();

        if GetSystemTimes(&mut idle, &mut kernel, &mut user) == 0 {
            return None;
        }

        let idle = filetime_to_u64(idle);
        let kernel = filetime_to_u64(kernel);
        let user = filetime_to_u64(user);

        Some(CpuTimes {
            idle,
            total: kernel.saturating_add(user),
        })
    }
}

fn filetime_to_u64(time: FILETIME) -> u64 {
    ((time.dwHighDateTime as u64) << 32) | time.dwLowDateTime as u64
}

fn cpu_percent(previous: &CpuTimes, current: &CpuTimes) -> f32 {
    let idle = current.idle.saturating_sub(previous.idle);
    let total = current.total.saturating_sub(previous.total);

    if total == 0 {
        return 0.0;
    }

    let busy = total.saturating_sub(idle);

    (busy as f32 / total as f32) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_reader_returns_the_provided_stats() {
        let expected = SystemStats {
            cpu_percent: 42.0,
            ram_used_bytes: 1_000,
            ram_total_bytes: 2_000,
            gpu_percent: Some(15.0),
            gpu_memory_used_bytes: Some(500),
            gpu_memory_total_bytes: Some(8_000),
            cpu_temp_celsius: Some(61.0),
            gpu_temp_celsius: Some(70.0),
        };

        let mut service = SystemStatsService::fixed(expected.clone());

        assert_eq!(service.collect(), expected);
    }

    #[test]
    fn cpu_percent_uses_idle_and_total_deltas() {
        let previous = CpuTimes {
            idle: 100,
            total: 200,
        };

        let current = CpuTimes {
            idle: 150,
            total: 300,
        };

        assert_eq!(cpu_percent(&previous, &current), 50.0);
    }

    #[test]
    fn reads_memory_from_the_host() {
        let mut service = SystemStatsService::new();

        let stats = service.collect();

        assert!(
            stats.ram_total_bytes > 0,
            "expected the host to report installed memory"
        );
    }
}
