mod app;
mod commands;
mod schemas;
mod services;
mod state;
mod adapters;
mod devices;
mod factories;

#[cfg(test)]
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    app::run();
}