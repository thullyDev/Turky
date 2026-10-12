use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

use crate::devices::device_id::DeviceId;
use crate::schemas::overlay_schemas::OverlayConfig;
use crate::services::device_service::DeviceService;
use crate::services::gif_cutter::GifFrame;
use crate::services::overlay_service::OverlayService;

use image::RgbaImage;

pub(crate) const STATS_INTERVAL: Duration = Duration::from_secs(2);

pub enum GifLaunch {
    Clip { width: u32, height: u32 },
    Animated(Vec<GifFrame>),
    Empty,
}

pub struct DisplayService {
    pub device_serv: DeviceService,
    overlay_service: OverlayService,
}

impl DisplayService {
    pub fn new(device_serv: DeviceService, overlay_service: OverlayService) -> Self {
        Self {
            device_serv,
            overlay_service,
        }
    }

    pub fn set_overlay_config(&mut self, config: OverlayConfig) {
        self.overlay_service.set_config(config);
    }

    pub fn render_image(&mut self, image: &RgbaImage, device_id: &DeviceId) {
        if let Some(device) = self.device_serv.registry().get_mut(device_id) {
            device
                .adapter
                .send_frame(image)
                .expect("Failed to send frame");
        }
    }

    pub fn render_gif(
        &mut self,
        bytes: &[u8],
        device_id: &DeviceId,
        presented: Option<&AtomicU32>,
    ) -> Result<(), String> {
        self.play_gif(bytes, device_id, None, presented)
    }

    pub fn launch_gif(
        &mut self,
        bytes: &[u8],
        device_id: &DeviceId,
    ) -> Result<GifLaunch, String> {
        let decoded = self.overlay_service.cut(bytes)?;

        if decoded.is_empty() || self.device_serv.registry().get(device_id).is_none() {
            return Ok(GifLaunch::Empty);
        }

        let frames: Vec<&RgbaImage> = decoded.iter().map(|frame| &frame.image).collect();
        let delays: Vec<Duration> = decoded.iter().map(|frame| frame.delay).collect();

        let published = {
            let Some(device) = self.device_serv.registry().get_mut(device_id) else {
                return Ok(GifLaunch::Empty);
            };

            device.adapter.reset_preloaded();

            device
                .adapter
                .publish_clip(&frames, &delays)
                .map_err(|error| format!("Failed to publish clip: {error:?}"))?
        };

        if published {
            let width = decoded[0].image.width();
            let height = decoded[0].image.height();

            return Ok(GifLaunch::Clip { width, height });
        }

        Ok(GifLaunch::Animated(decoded))
    }

    pub fn publish_stats_overlay(
        &mut self,
        device_id: &DeviceId,
        width: u32,
        height: u32,
        presented: Option<&AtomicU32>,
        sampled_at: &mut Instant,
    ) -> Result<(), String> {
        self.sample_presented(presented, sampled_at);

        let layer = self.overlay_service.refresh_layer(width, height);

        self.publish_overlay(device_id, layer.as_ref())
    }

    fn play_gif(
        &mut self,
        bytes: &[u8],
        device_id: &DeviceId,
        repeats: Option<usize>,
        presented: Option<&AtomicU32>,
    ) -> Result<(), String> {
        match self.launch_gif(bytes, device_id)? {
            GifLaunch::Empty => Ok(()),
            GifLaunch::Clip { width, height } => {
                self.play_clip(device_id, width, height, repeats, presented)
            }
            GifLaunch::Animated(frames) => self.play_frames(&frames, device_id, repeats, &|| false),
        }
    }

    fn play_clip(
        &mut self,
        device_id: &DeviceId,
        width: u32,
        height: u32,
        repeats: Option<usize>,
        presented: Option<&AtomicU32>,
    ) -> Result<(), String> {
        let mut sampled_at = Instant::now();
        let mut cycles = 0;

        loop {
            self.publish_stats_overlay(device_id, width, height, presented, &mut sampled_at)?;

            cycles += 1;

            if repeats.is_some_and(|limit| cycles >= limit) {
                break;
            }

            std::thread::sleep(STATS_INTERVAL);
        }

        Ok(())
    }

    fn sample_presented(&mut self, presented: Option<&AtomicU32>, sampled_at: &mut Instant) {
        let Some(presented) = presented else {
            *sampled_at = Instant::now();

            return;
        };

        let frames = presented.swap(0, Ordering::Relaxed);
        let seconds = sampled_at.elapsed().as_secs_f32();

        *sampled_at = Instant::now();

        if seconds >= 0.05 && frames > 0 {
            self.overlay_service
                .set_fps(Some(frames as f32 / seconds));
        }
    }

    fn publish_overlay(
        &mut self,
        device_id: &DeviceId,
        overlay: Option<&RgbaImage>,
    ) -> Result<(), String> {
        let Some(device) = self.device_serv.registry().get_mut(device_id) else {
            return Ok(());
        };

        device
            .adapter
            .publish_overlay(overlay)
            .map_err(|error| format!("Failed to publish overlay: {error:?}"))
    }

    pub fn play_frames_until(
        &mut self,
        decoded: &[GifFrame],
        device_id: &DeviceId,
        stop: &dyn Fn() -> bool,
    ) -> Result<(), String> {
        self.play_frames(decoded, device_id, None, stop)
    }

    fn play_frames(
        &mut self,
        decoded: &[GifFrame],
        device_id: &DeviceId,
        repeats: Option<usize>,
        stop: &dyn Fn() -> bool,
    ) -> Result<(), String> {
        let _timer = PreciseWait::start();
        let mut played = 0;
        let mut next = Instant::now();

        loop {
            for gif_frame in decoded {
                if stop() {
                    return Ok(());
                }

                let frame = self.overlay_service.compose(&gif_frame.image);

                self.render_image(&frame, device_id);

                next += gif_frame.delay;

                let now = Instant::now();

                if now < next {
                    wait_until(next);
                } else {
                    next = pace_deadline(next, now, gif_frame.delay);
                }
            }

            played += 1;

            if repeats.is_some_and(|limit| played >= limit) {
                break;
            }
        }

        Ok(())
    }
}

struct PreciseWait;

impl PreciseWait {
    fn start() -> Self {
        #[cfg(windows)]
        unsafe {
            winapi::um::timeapi::timeBeginPeriod(1);
        }

        Self
    }
}

impl Drop for PreciseWait {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            winapi::um::timeapi::timeEndPeriod(1);
        }
    }
}

fn wait_until(deadline: Instant) {
    loop {
        let now = Instant::now();

        if now >= deadline {
            return;
        }

        let remaining = deadline - now;

        if remaining > Duration::from_millis(2) {
            std::thread::sleep(remaining - Duration::from_millis(1));
        } else {
            std::hint::spin_loop();
        }
    }
}

fn pace_deadline(scheduled: Instant, now: Instant, delay: Duration) -> Instant {
    if now <= scheduled {
        return scheduled;
    }

    let late = now.saturating_duration_since(scheduled);

    if late < delay {
        scheduled
    } else {
        now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use gif::{Encoder, Frame, Repeat};
    use image::Rgba;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use image::RgbaImage;

    use crate::adapters::display_adapter::{AdapterError, DisplayAdapter};
    use crate::{
        adapters::usbs::{
            usb_adapter::UsbAdapter, usb_connection::UsbConnection, usb_device_info::UsbDeviceInfo,
            usb_errors::UsbError,
        },
        devices::{device_registry::DeviceRegistry, device_session::DeviceSession},
        factories::device_factory::DeviceFactory,
        services::{
            device_service::DeviceService, gif_cutter::GifCutter, overlay_service::OverlayService,
            stats_painter::StatsPainter, system_stats_service::SystemStatsService,
        },
        utils::test_utils::{get_test_device_info, FakeDisplayAdapter},
    };

    use crate::services::system_stats_service::SystemStats;

    struct FakeUsbConnection;

    impl UsbConnection for FakeUsbConnection {
        fn open(&mut self) -> Result<(), UsbError> {
            Ok(())
        }

        fn close(&mut self) {}

        fn write(&mut self, _endpoint: u8, data: &[u8]) -> Result<usize, UsbError> {
            Ok(data.len())
        }

        fn read(&mut self, _endpoint: u8, data: &mut [u8]) -> Result<usize, UsbError> {
            Ok(data.len())
        }
    }

    struct FakeUsbAdapter {
        devices: Vec<UsbDeviceInfo>,
    }

    impl UsbAdapter for FakeUsbAdapter {
        fn discover_devices(&self) -> Result<Vec<UsbDeviceInfo>, UsbError> {
            Ok(self.devices.clone())
        }

        fn connect(&self, _device: &UsbDeviceInfo) -> Result<Box<dyn UsbConnection>, UsbError> {
            Ok(Box::new(FakeUsbConnection))
        }
    }

    fn create_test_service(sent_frames: Arc<Mutex<Vec<Vec<u8>>>>) -> DisplayService {
        let mut registry = DeviceRegistry::new();

        let info = get_test_device_info();

        let adapter = FakeDisplayAdapter {
            info: info.clone(),
            connected: true,
            sent_frames,
        };

        let session = DeviceSession::new(Box::new(adapter));

        registry.add(info.id.clone(), session);

        let factory = DeviceFactory::new();

        let usb = FakeUsbAdapter { devices: vec![] };

        let device_service = DeviceService::new_for_test(registry, factory, Box::new(usb));

        DisplayService::new(device_service, test_overlay())
    }

    fn test_overlay() -> OverlayService {
        OverlayService::new(
            GifCutter::new(),
            SystemStatsService::fixed(SystemStats {
                cpu_percent: 100.0,
                ..SystemStats::empty()
            }),
            StatsPainter::new(),
        )
    }

    #[test]
    fn creates_display_service() {
        let registry = DeviceRegistry::new();

        let factory = DeviceFactory::new();

        let usb = FakeUsbAdapter { devices: vec![] };

        let device_service = DeviceService::new_for_test(registry, factory, Box::new(usb));

        let _service = DisplayService::new(device_service, test_overlay());
    }

    #[test]
    fn renders_image_when_device_exists() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(Arc::clone(&sent_frames));

        let image = RgbaImage::from_pixel(480, 1920, Rgba([255, 0, 0, 255]));

        let device_id = get_test_device_info().id.clone();

        service.render_image(&image, &device_id);

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 1);

        assert_eq!(frames[0].len(), 480 * 1920 * 4);
    }

    #[test]
    fn does_not_panic_when_no_device_exists() {
        let registry = DeviceRegistry::new();

        let factory = DeviceFactory::new();

        let usb = FakeUsbAdapter { devices: vec![] };

        let device_service = DeviceService::new_for_test(registry, factory, Box::new(usb));

        let mut service = DisplayService::new(device_service, test_overlay());

        let image = RgbaImage::from_pixel(480, 1920, Rgba([255, 0, 0, 255]));

        let device_id = get_test_device_info().id.clone();

        service.render_image(&image, &device_id);
    }

    fn create_test_gif() -> Vec<u8> {
        let mut bytes = Vec::new();

        {
            let mut encoder =
                Encoder::new(&mut bytes, 2, 2, &[]).expect("Failed to create GIF encoder");

            encoder
                .set_repeat(Repeat::Infinite)
                .expect("Failed to set GIF repeat");

            let red = vec![255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];

            let mut red_frame = Frame::from_rgb(2, 2, &red);

            red_frame.delay = 1;

            encoder
                .write_frame(&red_frame)
                .expect("Failed to write red frame");

            let blue = vec![0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0, 255];

            let mut blue_frame = Frame::from_rgb(2, 2, &blue);

            blue_frame.delay = 1;

            encoder
                .write_frame(&blue_frame)
                .expect("Failed to write blue frame");
        }

        bytes
    }

    #[test]
    fn renders_gif_frames_when_device_exists() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(Arc::clone(&sent_frames));

        let device_id = get_test_device_info().id.clone();

        let gif = create_test_gif();

        let result = service.play_gif(&gif, &device_id, Some(1), None);

        assert!(result.is_ok(), "render_gif failed: {:?}", result.err());

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 2, "Expected 2 GIF frames to be rendered");

        assert_eq!(frames[0].len(), 2 * 2 * 4);

        assert_eq!(frames[1].len(), 2 * 2 * 4);
    }

    #[test]
    fn gif_frames_include_the_current_stats() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));
        let mut service = create_test_service(Arc::clone(&sent_frames));
        let mut bytes = Vec::new();

        {
            let mut encoder =
                Encoder::new(&mut bytes, 320, 80, &[]).expect("Failed to create GIF encoder");

            encoder
                .set_repeat(Repeat::Infinite)
                .expect("Failed to set GIF repeat");

            let pixels = vec![0u8; 320 * 80 * 3];
            let mut frame = Frame::from_rgb(320, 80, &pixels);

            frame.delay = 1;

            encoder
                .write_frame(&frame)
                .expect("Failed to write GIF frame");
        }

        service
            .play_gif(&bytes, &get_test_device_info().id, Some(1), None)
            .expect("gif should play");

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 1);

        let has_text = frames[0]
            .chunks(4)
            .any(|pixel| pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200);

        assert!(has_text, "expected current stats on the sent GIF frame");
    }

    #[test]
    fn render_gif_returns_error_for_invalid_data() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(sent_frames);

        let device_id = get_test_device_info().id.clone();

        let invalid_gif = b"this is not a gif";

        let result = service.render_gif(invalid_gif, &device_id, None);

        assert!(result.is_err(), "Expected invalid GIF to return an error");

        let error = result.unwrap_err();

        assert!(
            error.contains("Failed to create GIF decoder"),
            "Unexpected error: {error}"
        );
    }

    #[test]
    fn playback_waits_for_each_gif_frame_delay() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(sent_frames);

        let device_id = get_test_device_info().id.clone();

        let mut bytes = Vec::new();

        {
            let mut encoder =
                Encoder::new(&mut bytes, 2, 2, &[]).expect("Failed to create GIF encoder");

            encoder
                .set_repeat(Repeat::Infinite)
                .expect("Failed to set GIF repeat");

            let mut frame = Frame::from_rgb(2, 2, &[255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0]);

            frame.delay = 5;

            encoder
                .write_frame(&frame)
                .expect("Failed to write GIF frame");

            let mut frame = Frame::from_rgb(2, 2, &[0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0, 255]);

            frame.delay = 5;

            encoder
                .write_frame(&frame)
                .expect("Failed to write GIF frame");
        }

        let start = Instant::now();

        service
            .play_gif(&bytes, &device_id, Some(1), None)
            .expect("render_gif failed");

        let elapsed = start.elapsed();

        assert!(
            elapsed >= Duration::from_millis(90),
            "GIF rendered too quickly: {:?}",
            elapsed
        );
    }

    #[test]
    fn render_gif_with_missing_device_does_not_send_frames() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(Arc::clone(&sent_frames));

        let device_id = DeviceId("does-not-exist".to_string());

        let gif = create_test_gif();

        let result = service.render_gif(&gif, &device_id, None);

        assert!(result.is_ok(), "Expected render_gif to complete");

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 0, "No frames should have been sent");
    }

    struct RecordingClipAdapter {
        info: crate::devices::device_info::DeviceInfo,
        clips: Arc<Mutex<Vec<usize>>>,
        overlays: Arc<Mutex<usize>>,
        sent_frames: Arc<Mutex<usize>>,
    }

    impl DisplayAdapter for RecordingClipAdapter {
        fn info(&self) -> &crate::devices::device_info::DeviceInfo {
            &self.info
        }

        fn connect(&mut self) -> Result<(), AdapterError> {
            Ok(())
        }

        fn disconnect(&mut self) -> Result<(), AdapterError> {
            Ok(())
        }

        fn send_frame(&mut self, _image: &RgbaImage) -> Result<(), AdapterError> {
            *self.sent_frames.lock().unwrap() += 1;

            Ok(())
        }

        fn publish_clip(
            &mut self,
            frames: &[&RgbaImage],
            _delays: &[Duration],
        ) -> Result<bool, AdapterError> {
            self.clips.lock().unwrap().push(frames.len());

            Ok(true)
        }

        fn publish_overlay(&mut self, _overlay: Option<&RgbaImage>) -> Result<(), AdapterError> {
            *self.overlays.lock().unwrap() += 1;

            Ok(())
        }

        fn is_connected(&self) -> bool {
            true
        }

        fn clear(&mut self) -> Result<(), AdapterError> {
            Ok(())
        }
    }

    #[test]
    fn clip_playback_publishes_once_and_does_not_send_each_frame() {
        let clips = Arc::new(Mutex::new(Vec::<usize>::new()));
        let overlays = Arc::new(Mutex::new(0usize));
        let sent_frames = Arc::new(Mutex::new(0usize));
        let mut registry = DeviceRegistry::new();
        let info = get_test_device_info();
        let adapter = RecordingClipAdapter {
            info: info.clone(),
            clips: Arc::clone(&clips),
            overlays: Arc::clone(&overlays),
            sent_frames: Arc::clone(&sent_frames),
        };

        registry.add(info.id.clone(), DeviceSession::new(Box::new(adapter)));

        let device_service = DeviceService::new_for_test(
            registry,
            DeviceFactory::new(),
            Box::new(FakeUsbAdapter { devices: vec![] }),
        );
        let mut service = DisplayService::new(device_service, test_overlay());
        let start = Instant::now();

        service
            .play_gif(&create_test_gif(), &info.id, Some(1), None)
            .expect("clip should play");

        assert!(
            start.elapsed() < Duration::from_millis(500),
            "clip playback should return without sleeping through the gif"
        );
        assert_eq!(clips.lock().unwrap().as_slice(), &[2]);
        assert_eq!(*overlays.lock().unwrap(), 1);
        assert_eq!(*sent_frames.lock().unwrap(), 0);
    }

    #[test]
    fn a_frame_that_is_only_slightly_late_keeps_its_deadline() {
        let start = Instant::now();
        let scheduled = start + Duration::from_millis(50);
        let now = start + Duration::from_millis(60);

        assert_eq!(
            pace_deadline(scheduled, now, Duration::from_millis(50)),
            scheduled
        );
    }

    #[test]
    fn a_frame_that_is_a_full_interval_late_resets_the_deadline() {
        let start = Instant::now();
        let scheduled = start + Duration::from_millis(50);
        let now = start + Duration::from_millis(120);

        assert_eq!(
            pace_deadline(scheduled, now, Duration::from_millis(50)),
            now
        );
    }
}
