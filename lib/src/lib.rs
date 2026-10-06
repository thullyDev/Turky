mod adapters;
mod app;
mod commands;
mod devices;
mod factories;
mod schemas;
mod services;
mod state;

#[cfg(test)]
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    app::run();
}
