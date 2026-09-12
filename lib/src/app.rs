use crate::adapters::usbs::rusb_adapter::RusbAdapter;
use crate::commands;
use crate::devices::device_registry::DeviceRegistry;
use crate::factories::device_factory::DeviceFactory;
use crate::services;
use crate::state::AppState;

pub fn run() {
    let usb = Box::new(RusbAdapter::new());
    let registry = DeviceRegistry::new();
    let factory = DeviceFactory::new();
    let mut device_service = services::device_service::DeviceService::new(registry, factory, usb);
    device_service.discover_devices();
    let display_service = services::display_service::DisplayService::new(device_service);
    let state = AppState::new(display_service);
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state);

    commands::register(builder)
        .run(tauri::generate_context!())
        .expect("error while running turky");
}
