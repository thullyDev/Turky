use std::io::Cursor;

use image::{codecs::gif::GifDecoder, AnimationDecoder, RgbaImage};

use crate::devices::device_id::DeviceId;
use crate::rendering::OverlayRenderer;
use crate::schemas::overlay_schemas::OverlayConfig;
use crate::services::device_service::DeviceService;
use crate::services::system_stats_service::SystemStatsService;

#[cfg(test)]
use crate::services::system_stats_service::SystemStats;

pub struct DisplayService {
    pub device_serv: DeviceService,
    overlay_renderer: OverlayRenderer,
    stats_service: SystemStatsService,
    config: OverlayConfig,
}

impl DisplayService {
    pub fn new(
        device_serv: DeviceService,
        overlay_renderer: OverlayRenderer,
        stats_service: SystemStatsService,
    ) -> Self {
        Self {
            device_serv,
            overlay_renderer,
            stats_service,
            config: OverlayConfig::default(),
        }
    }

    pub fn set_overlay_config(&mut self, config: OverlayConfig) {
        self.config = config;
    }

    pub fn register_font(&mut self, name: String, bytes: Vec<u8>) -> Result<(), String> {
        self.overlay_renderer.register_font(name, bytes)
    }

    pub fn font_names(&self) -> Vec<String> {
        self.overlay_renderer.font_names()
    }

    pub fn render_image(&mut self, image: &RgbaImage, device_id: &DeviceId) {
        if let Some(device) = self.device_serv.registry().get_mut(device_id) {
            device.adapter.dismiss_gif();
        }

        if self.config.items.is_empty() {
            self.send(image, device_id);

            return;
        }

        let stats = self.stats_service.collect();
        let frame = self.overlay_renderer.render(image, &stats, &self.config);

        self.send(&frame, device_id);
    }

    fn send(&mut self, image: &RgbaImage, device_id: &DeviceId) {
        if let Some(device) = self.device_serv.registry().get_mut(device_id) {
            device
                .adapter
                .send_frame(image)
                .expect("Failed to send frame");
        }
    }

    pub fn render_gif(&mut self, bytes: &[u8], device_id: &DeviceId) -> Result<(), String> {
        self.play_gif(bytes, device_id, None)
    }

    fn play_gif(
        &mut self,
        bytes: &[u8],
        device_id: &DeviceId,
        repeats: Option<usize>,
    ) -> Result<(), String> {
        let decoder = GifDecoder::new(Cursor::new(bytes))
            .map_err(|e| format!("Failed to create GIF decoder: {e}"))?;

        if self.device_serv.registry().get(device_id).is_none() {
            return Ok(());
        }

        let presented = self
            .device_serv
            .registry()
            .get_mut(device_id)
            .expect("display exists")
            .adapter
            .present_gif(bytes)
            .map_err(|error| format!("Failed to show GIF: {error:?}"))?;

        if presented {
            return self.play_stats_over_gif(device_id, repeats);
        }

        let frame_duration = std::time::Duration::from_secs_f64(1.0 / 60.0);
        let mut frames = decoder.into_frames();
        let mut decoded = Vec::new();

        loop {
            let Some(frame_result) = frames.next() else {
                break;
            };

            let frame = frame_result.map_err(|e| format!("Failed to decode GIF frame: {e}"))?;

            decoded.push(frame.into_buffer());
        }

        if decoded.is_empty() || self.device_serv.registry().get(device_id).is_none() {
            return Ok(());
        }

        let preload_frames = self.config.items.is_empty();

        if let Some(device) = self.device_serv.registry().get_mut(device_id) {
            device.adapter.reset_preloaded();

            if preload_frames {
                for image in &decoded {
                    device
                        .adapter
                        .preload_frame(image)
                        .map_err(|error| format!("Failed to prepare frame: {error:?}"))?;
                }
            }
        }

        let mut played = 0;

        loop {
            for image in &decoded {
                let start = std::time::Instant::now();

                self.render_image(image, device_id);

                let elapsed = start.elapsed();

                if elapsed < frame_duration {
                    std::thread::sleep(frame_duration - elapsed);
                }
            }

            played += 1;

            if repeats.is_some_and(|limit| played >= limit) {
                break;
            }
        }

        Ok(())
    }

    fn play_stats_over_gif(
        &mut self,
        device_id: &DeviceId,
        repeats: Option<usize>,
    ) -> Result<(), String> {
        if self.config.items.is_empty() {
            return Ok(());
        }

        let (width, height) = {
            let device = self
                .device_serv
                .registry()
                .get(device_id)
                .expect("display exists");

            (device.info.width, device.info.height)
        };

        let frame_duration = std::time::Duration::from_secs_f64(1.0 / 60.0);
        let mut played = 0;

        loop {
            let start = std::time::Instant::now();
            let stats = self.stats_service.collect();

            if let Some(layer) = self
                .overlay_renderer
                .render_layer(width, height, &stats, &self.config)
            {
                self.send(&layer, device_id);
            }

            let elapsed = start.elapsed();

            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }

            played += 1;

            if repeats.is_some_and(|limit| played >= limit) {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use gif::{Encoder, Frame, Repeat};
    use image::Rgba;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use crate::{
        adapters::{
            display_adapter::{AdapterError, DisplayAdapter},
            usbs::{
                usb_adapter::UsbAdapter, usb_connection::UsbConnection,
                usb_device_info::UsbDeviceInfo, usb_errors::UsbError,
            },
        },
        devices::{
            device_info::DeviceInfo, device_registry::DeviceRegistry, device_session::DeviceSession,
        },
        factories::device_factory::DeviceFactory,
        rendering::{FontRegistry, OverlayFormatter, OverlayRenderer, TextLayout},
        schemas::overlay_schemas::{
            OverlayColor, OverlayItem, OverlayMetric, OverlayPosition, TextAnchor,
        },
        services::device_service::DeviceService,
        utils::test_utils::{get_test_device_info, FakeDisplayAdapter},
    };

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

        display_service(device_service, SystemStatsService::new())
    }

    #[test]
    fn creates_display_service() {
        let registry = DeviceRegistry::new();

        let factory = DeviceFactory::new();

        let usb = FakeUsbAdapter { devices: vec![] };

        let device_service = DeviceService::new_for_test(registry, factory, Box::new(usb));

        let _service = display_service(device_service, SystemStatsService::new());
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

        let mut service = display_service(device_service, SystemStatsService::new());

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

        let result = service.play_gif(&gif, &device_id, Some(1));

        assert!(result.is_ok(), "render_gif failed: {:?}", result.err());

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 2, "Expected 2 GIF frames to be rendered");

        assert_eq!(frames[0].len(), 2 * 2 * 4);

        assert_eq!(frames[1].len(), 2 * 2 * 4);
    }

    #[test]
    fn render_gif_returns_error_for_invalid_data() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(sent_frames);

        let device_id = get_test_device_info().id.clone();

        let invalid_gif = b"this is not a gif";

        let result = service.render_gif(invalid_gif, &device_id);

        assert!(result.is_err(), "Expected invalid GIF to return an error");

        let error = result.unwrap_err();

        assert!(
            error.contains("Failed to create GIF decoder"),
            "Unexpected error: {error}"
        );
    }

    #[test]
    fn render_gif_targets_30_fps() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service(sent_frames);

        let device_id = get_test_device_info().id.clone();

        let gif = create_test_gif();

        let start = Instant::now();

        service
            .play_gif(&gif, &device_id, Some(1))
            .expect("render_gif failed");

        let elapsed = start.elapsed();

        assert!(
            elapsed >= Duration::from_millis(60),
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

        let result = service.render_gif(&gif, &device_id);

        assert!(result.is_ok(), "Expected render_gif to complete");

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 0, "No frames should have been sent");
    }

    fn cpu_overlay() -> OverlayConfig {
        OverlayConfig {
            items: vec![OverlayItem {
                metric: OverlayMetric::Cpu,
                position: OverlayPosition {
                    x: 8,
                    y: 8,
                    anchor: TextAnchor::TopLeft,
                },
                font: "Roboto".to_string(),
                size: 28.0,
                color: OverlayColor {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
                text: String::new(),
            }],
        }
    }

    #[test]
    fn render_image_draws_overlay_text_before_send() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        let mut service = create_test_service_with_stats(
            Arc::clone(&sent_frames),
            SystemStats {
                cpu_percent: 100.0,
                ..SystemStats::empty()
            },
        );

        service.set_overlay_config(cpu_overlay());

        let image = RgbaImage::from_pixel(320, 80, Rgba([0, 0, 0, 255]));
        let device_id = get_test_device_info().id.clone();

        service.render_image(&image, &device_id);

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].len(), 320 * 80 * 4);

        let has_text = frames[0]
            .chunks(4)
            .any(|pixel| pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200);

        assert!(has_text, "expected white overlay text on the sent frame");
    }

    struct RecordingAdapter {
        info: DeviceInfo,
        sent_frames: Arc<Mutex<Vec<Vec<u8>>>>,
        preloaded: Arc<Mutex<usize>>,
    }

    impl DisplayAdapter for RecordingAdapter {
        fn info(&self) -> &DeviceInfo {
            &self.info
        }

        fn connect(&mut self) -> Result<(), AdapterError> {
            Ok(())
        }

        fn disconnect(&mut self) -> Result<(), AdapterError> {
            Ok(())
        }

        fn send_frame(&mut self, image: &RgbaImage) -> Result<(), AdapterError> {
            self.sent_frames
                .lock()
                .unwrap()
                .push(image.as_raw().clone());

            Ok(())
        }

        fn preload_frame(&mut self, _image: &RgbaImage) -> Result<(), AdapterError> {
            *self.preloaded.lock().unwrap() += 1;

            Ok(())
        }

        fn clear(&mut self) -> Result<(), AdapterError> {
            Ok(())
        }

        fn is_connected(&self) -> bool {
            true
        }
    }

    fn create_recording_service(
        sent_frames: Arc<Mutex<Vec<Vec<u8>>>>,
        preloaded: Arc<Mutex<usize>>,
        stats_service: SystemStatsService,
    ) -> DisplayService {
        let mut registry = DeviceRegistry::new();
        let info = get_test_device_info();

        let session = DeviceSession::new(Box::new(RecordingAdapter {
            info: info.clone(),
            sent_frames,
            preloaded,
        }));

        registry.add(info.id.clone(), session);

        let device_service = DeviceService::new_for_test(
            registry,
            DeviceFactory::new(),
            Box::new(FakeUsbAdapter { devices: vec![] }),
        );

        display_service(device_service, stats_service)
    }

    fn display_service(
        device_service: DeviceService,
        stats_service: SystemStatsService,
    ) -> DisplayService {
        DisplayService::new(
            device_service,
            OverlayRenderer::new(
                FontRegistry::bundled(),
                OverlayFormatter::new(),
                TextLayout::new(),
            ),
            stats_service,
        )
    }

    fn create_test_service_with_stats(
        sent_frames: Arc<Mutex<Vec<Vec<u8>>>>,
        stats: SystemStats,
    ) -> DisplayService {
        let mut registry = DeviceRegistry::new();
        let info = get_test_device_info();

        let session = DeviceSession::new(Box::new(FakeDisplayAdapter {
            info: info.clone(),
            connected: true,
            sent_frames,
        }));

        registry.add(info.id.clone(), session);

        let device_service = DeviceService::new_for_test(
            registry,
            DeviceFactory::new(),
            Box::new(FakeUsbAdapter { devices: vec![] }),
        );

        display_service(device_service, SystemStatsService::fixed(stats))
    }

    #[test]
    fn gif_without_overlays_preloads_frames() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));
        let preloaded = Arc::new(Mutex::new(0usize));

        let mut service = create_recording_service(
            Arc::clone(&sent_frames),
            Arc::clone(&preloaded),
            SystemStatsService::new(),
        );

        service
            .play_gif(&create_test_gif(), &get_test_device_info().id, Some(1))
            .expect("gif should play");

        assert_eq!(*preloaded.lock().unwrap(), 2);
        assert_eq!(sent_frames.lock().unwrap().len(), 2);
    }

    #[test]
    fn gif_with_overlays_skips_preload_and_draws_text() {
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));
        let preloaded = Arc::new(Mutex::new(0usize));

        let mut service = create_recording_service(
            Arc::clone(&sent_frames),
            Arc::clone(&preloaded),
            SystemStatsService::fixed(SystemStats {
                cpu_percent: 100.0,
                ..SystemStats::empty()
            }),
        );

        service.set_overlay_config(cpu_overlay());

        let mut bytes = Vec::new();

        {
            let mut encoder =
                Encoder::new(&mut bytes, 64, 32, &[]).expect("Failed to create GIF encoder");

            encoder
                .set_repeat(Repeat::Infinite)
                .expect("Failed to set GIF repeat");

            let pixels = vec![0u8; 64 * 32 * 3];
            let mut frame = Frame::from_rgb(64, 32, &pixels);

            frame.delay = 1;

            encoder
                .write_frame(&frame)
                .expect("Failed to write GIF frame");
        }

        service
            .play_gif(&bytes, &get_test_device_info().id, Some(1))
            .expect("gif should play");

        assert_eq!(*preloaded.lock().unwrap(), 0);

        let frames = sent_frames.lock().unwrap();

        assert_eq!(frames.len(), 1);

        let has_text = frames[0]
            .chunks(4)
            .any(|pixel| pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200);

        assert!(has_text, "expected overlay text on the GIF frame");
    }

    struct HtmlGifAdapter {
        info: DeviceInfo,
        gif: Arc<Mutex<Vec<u8>>>,
        sent_frames: Arc<Mutex<Vec<Vec<u8>>>>,
    }

    impl DisplayAdapter for HtmlGifAdapter {
        fn info(&self) -> &DeviceInfo {
            &self.info
        }

        fn connect(&mut self) -> Result<(), AdapterError> {
            Ok(())
        }

        fn disconnect(&mut self) -> Result<(), AdapterError> {
            Ok(())
        }

        fn send_frame(&mut self, image: &RgbaImage) -> Result<(), AdapterError> {
            self.sent_frames
                .lock()
                .unwrap()
                .push(image.as_raw().clone());

            Ok(())
        }

        fn present_gif(&mut self, bytes: &[u8]) -> Result<bool, AdapterError> {
            *self.gif.lock().unwrap() = bytes.to_vec();

            Ok(true)
        }

        fn clear(&mut self) -> Result<(), AdapterError> {
            Ok(())
        }

        fn is_connected(&self) -> bool {
            true
        }
    }

    fn create_html_gif_service(
        gif: Arc<Mutex<Vec<u8>>>,
        sent_frames: Arc<Mutex<Vec<Vec<u8>>>>,
        stats: SystemStats,
    ) -> DisplayService {
        let mut registry = DeviceRegistry::new();
        let info = get_test_device_info();

        let session = DeviceSession::new(Box::new(HtmlGifAdapter {
            info: info.clone(),
            gif,
            sent_frames,
        }));

        registry.add(info.id.clone(), session);

        let device_service = DeviceService::new_for_test(
            registry,
            DeviceFactory::new(),
            Box::new(FakeUsbAdapter { devices: vec![] }),
        );

        display_service(device_service, SystemStatsService::fixed(stats))
    }

    #[test]
    fn gif_file_is_shown_in_html_and_stats_render_on_top() {
        let gif = Arc::new(Mutex::new(Vec::<u8>::new()));
        let sent_frames = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));
        let bytes = create_test_gif();

        let mut service = create_html_gif_service(
            Arc::clone(&gif),
            Arc::clone(&sent_frames),
            SystemStats {
                cpu_percent: 100.0,
                ..SystemStats::empty()
            },
        );

        service.set_overlay_config(cpu_overlay());

        service
            .play_gif(&bytes, &get_test_device_info().id, Some(1))
            .expect("gif should play");

        assert_eq!(gif.lock().unwrap().as_slice(), bytes.as_slice());

        let frames = sent_frames.lock().unwrap();
        let info = get_test_device_info();

        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].len(), (info.width * info.height * 4) as usize);
        assert_eq!(&frames[0][frames[0].len() - 4..], &[0, 0, 0, 0]);

        let has_text = frames[0]
            .chunks(4)
            .any(|pixel| pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200);

        assert!(has_text, "expected stats text on the transparent overlay");
    }
}
