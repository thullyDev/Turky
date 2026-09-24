use tauri::Manager;

use crate::adapters::usbs::rusb_adapter::RusbAdapter;
use crate::commands;
use crate::devices::device_registry::DeviceRegistry;
use crate::factories::device_factory::DeviceFactory;
use crate::services;
use crate::state::AppState;

pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    let virtual_device = args.iter().any(|arg| arg == "--virtual-device");

    let width = args.iter().find_map(|arg| {
        arg.strip_prefix("--width=")
            .and_then(|value| value.parse::<u32>().ok())
    });

    let height = args.iter().find_map(|arg| {
        arg.strip_prefix("--height=")
            .and_then(|value| value.parse::<u32>().ok())
    });

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let usb = Box::new(RusbAdapter::new());
            let registry = DeviceRegistry::new();
            let factory = DeviceFactory::new();

            let mut device_service = services::device_service::DeviceService::new(
                registry,
                factory,
                usb,
                app.handle().clone(),
            );

            if virtual_device {
                let width = width.expect("--width is required");
                let height = height.expect("--height is required");

                device_service.use_virtual_device(width, height);
            } else {
                device_service.discover_devices();
            }

            let display_service = services::display_service::DisplayService::new(device_service);

            let state = AppState::new(display_service);

            app.manage(state);

            Ok(())
        });

    commands::register(builder)
        .run(tauri::generate_context!())
        .expect("error while running turky");
}
