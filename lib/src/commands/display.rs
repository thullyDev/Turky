use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::services::display_service::{GifLaunch, STATS_INTERVAL};
use crate::state::AppState;

#[tauri::command]
pub fn render_image(
    request: tauri::ipc::Request<'_>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let bytes = owned_media(request)?;
    let image = image::load_from_memory(&bytes)
        .map_err(|error| format!("Failed to decode image: {error}"))?
        .to_rgba8();

    state.playback.fetch_add(1, Ordering::AcqRel);

    let mut display = state
        .display
        .lock()
        .map_err(|error| format!("Failed to lock DisplayService: {error}"))?;

    // TODO: Get the selected device ID from the frontend.
    let device_id = display
        .device_serv
        .registry()
        .first_mut()
        .map(|device| device.info.id.clone());

    if let Some(device_id) = device_id {
        display.render_image(&image, &device_id);
    } else {
        println!("No display device available");
    }

    Ok(())
}

#[tauri::command]
pub fn render_gif<R: Runtime>(
    app: AppHandle<R>,
    request: tauri::ipc::Request<'_>,
) -> Result<(), String> {
    let bytes = owned_media(request)?;
    let state = app.state::<AppState>();
    let generation = state.playback.fetch_add(1, Ordering::AcqRel) + 1;
    let playback = Arc::clone(&state.playback);
    let presented = Arc::clone(&state.presented);

    std::thread::Builder::new()
        .name("gif-playback".into())
        .spawn(move || {
            if let Err(error) =
                play_gif_in_background(&app, &bytes, generation, &playback, &presented)
            {
                let _ = app.emit("media-error", error);
            }
        })
        .map_err(|error| format!("Failed to start GIF playback: {error}"))?;

    Ok(())
}

#[tauri::command]
pub fn note_presented(frames: u32, state: tauri::State<'_, AppState>) {
    state.presented.fetch_add(frames, Ordering::Relaxed);
}

fn owned_media(request: tauri::ipc::Request<'_>) -> Result<Vec<u8>, String> {
    match request.body() {
        tauri::ipc::InvokeBody::Raw(bytes) => Ok(bytes.clone()),
        tauri::ipc::InvokeBody::Json(_) => Err("Media must be sent as bytes.".into()),
    }
}

fn play_gif_in_background<R: Runtime>(
    app: &AppHandle<R>,
    bytes: &[u8],
    generation: u64,
    playback: &AtomicU64,
    presented: &std::sync::atomic::AtomicU32,
) -> Result<(), String> {
    let state = app.state::<AppState>();
    let launch = {
        let mut display = state
            .display
            .lock()
            .map_err(|error| format!("Failed to lock DisplayService: {error}"))?;

        if playback.load(Ordering::Acquire) != generation {
            return Ok(());
        }

        let device_id = display
            .device_serv
            .registry()
            .first_mut()
            .map(|device| device.info.id.clone());

        let Some(device_id) = device_id else {
            return Err("No display device available".into());
        };

        match display.launch_gif(bytes, &device_id)? {
            GifLaunch::Empty => return Ok(()),
            GifLaunch::Clip { width, height } => StartedGif::Clip {
                device_id,
                width,
                height,
            },
            GifLaunch::Animated(frames) => StartedGif::Animated { device_id, frames },
        }
    };

    match launch {
        StartedGif::Clip {
            device_id,
            width,
            height,
        } => {
            let mut sampled_at = Instant::now();

            loop {
                if playback.load(Ordering::Acquire) != generation {
                    return Ok(());
                }

                {
                    let mut display = state
                        .display
                        .lock()
                        .map_err(|error| format!("Failed to lock DisplayService: {error}"))?;

                    if playback.load(Ordering::Acquire) != generation {
                        return Ok(());
                    }

                    display.publish_stats_overlay(
                        &device_id,
                        width,
                        height,
                        Some(presented),
                        &mut sampled_at,
                    )?;
                }

                if !wait_for_playback(playback, generation, STATS_INTERVAL) {
                    return Ok(());
                }
            }
        }
        StartedGif::Animated { device_id, frames } => {
            let mut display = state
                .display
                .lock()
                .map_err(|error| format!("Failed to lock DisplayService: {error}"))?;

            if playback.load(Ordering::Acquire) != generation {
                return Ok(());
            }

            display.play_frames_until(&frames, &device_id, &|| {
                playback.load(Ordering::Acquire) != generation
            })
        }
    }
}

enum StartedGif {
    Clip {
        device_id: crate::devices::device_id::DeviceId,
        width: u32,
        height: u32,
    },
    Animated {
        device_id: crate::devices::device_id::DeviceId,
        frames: Vec<crate::services::gif_cutter::GifFrame>,
    },
}

fn wait_for_playback(playback: &AtomicU64, generation: u64, total: Duration) -> bool {
    let step = Duration::from_millis(50);
    let mut left = total;

    while !left.is_zero() {
        if playback.load(Ordering::Acquire) != generation {
            return false;
        }

        let slice = left.min(step);

        std::thread::sleep(slice);
        left = left.saturating_sub(slice);
    }

    playback.load(Ordering::Acquire) == generation
}

#[cfg(test)]
mod tests {
    use image::{DynamicImage, Rgba, RgbaImage};
    use std::io::Cursor;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use super::wait_for_playback;

    #[test]
    fn decodes_valid_image_bytes() {
        let image = RgbaImage::from_pixel(480, 1920, Rgba([255, 0, 0, 255]));

        let mut bytes = Vec::new();

        DynamicImage::ImageRgba8(image)
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
            .expect("Failed to encode test image");

        let decoded = image::load_from_memory(&bytes)
            .expect("Failed to decode image")
            .to_rgba8();

        assert_eq!(decoded.width(), 480);
        assert_eq!(decoded.height(), 1920);

        assert_eq!(decoded.get_pixel(0, 0), &Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn rejects_invalid_image_bytes() {
        let bytes = vec![1, 2, 3, 4, 5];

        let result = image::load_from_memory(&bytes);

        assert!(result.is_err());
    }

    #[test]
    fn waiting_for_playback_stops_when_a_newer_gif_starts() {
        let playback = Arc::new(AtomicU64::new(1));
        let flag = Arc::clone(&playback);

        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(40));
            flag.store(2, Ordering::Release);
        });

        let started = Instant::now();
        let still_current = wait_for_playback(&playback, 1, Duration::from_secs(2));

        assert!(!still_current);
        assert!(
            started.elapsed() < Duration::from_millis(500),
            "cancel should interrupt the overlay wait"
        );
    }
}
