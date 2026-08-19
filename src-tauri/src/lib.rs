use std::time::Duration;

use ab_glyph::{FontRef, PxScale};
use cbc::cipher::{block_padding::NoPadding, BlockEncryptMut, KeyIvInit};
use chrono::Local;
use des::Des;
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusb::{DeviceHandle, GlobalContext};

const VENDOR_ID: u16 = 0x1CBE;
const PRODUCT_ID: u16 = 0x0088;

const IMAGE_WIDTH: u32 = 480;
const IMAGE_HEIGHT: u32 = 1920;

const USB_OUT: u8 = 0x01;
const USB_IN: u8 = 0x81;

const DES_KEY: &[u8; 8] = b"slv3tuzx";

type DesCbcEnc = cbc::Encryptor<Des>;

#[tauri::command]
fn send_message(message: &str) -> Result<String, String> {
    println!("📨 Received: {}", message);

    // --------------------------------------------------
    // Find TURZX
    // --------------------------------------------------

    let device = rusb::open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID)
        .ok_or_else(|| "TURZX1.0 not found".to_string())?;

    let handle = device;

    handle
        .set_active_configuration(1)
        .ok();

    handle
        .claim_interface(0)
        .map_err(|e| format!("Failed to claim USB interface: {}", e))?;

    println!("✅ TURZX1.0 found");

    // --------------------------------------------------
    // Build command
    // --------------------------------------------------

    fn build_command(command_id: u8) -> [u8; 500] {
        let mut packet = [0u8; 500];

        packet[0] = command_id;
        packet[2] = 0x1A;
        packet[3] = 0x6D;

        let now = Local::now();

        let midnight = now
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let milliseconds = (now.naive_local() - midnight)
            .num_milliseconds() as u32;

        packet[4..8].copy_from_slice(&milliseconds.to_le_bytes());

        packet
    }

    // --------------------------------------------------
    // DES-CBC encryption
    // --------------------------------------------------

    fn encrypt_command(packet: &[u8; 500]) -> [u8; 512] {
        let mut padded = [0u8; 504];

        padded[..500].copy_from_slice(packet);

        let mut encrypted = padded;

        let cipher = DesCbcEnc::new(
            DES_KEY.into(),
            DES_KEY.into(),
        );

        cipher
            .encrypt_padded_mut::<NoPadding>(
                &mut encrypted,
                504,
            )
            .unwrap();

        let mut result = [0u8; 512];

        result[..504].copy_from_slice(&encrypted);

        result[510] = 0xA1;
        result[511] = 0x1A;

        result
    }

    // --------------------------------------------------
    // USB send + response
    // --------------------------------------------------

    fn send_command(
        handle: &DeviceHandle<GlobalContext>,
        packet: &[u8],
        timeout: Duration,
    ) -> Result<Option<Vec<u8>>, String> {
        handle
            .write_bulk(USB_OUT, packet, timeout)
            .map_err(|e| format!("USB write failed: {}", e))?;

        let mut response = [0u8; 512];

        match handle.read_bulk(
            USB_IN,
            &mut response,
            timeout,
        ) {
            Ok(size) => Ok(Some(response[..size].to_vec())),

            Err(rusb::Error::Timeout) => Ok(None),

            Err(e) => Err(format!("USB read failed: {}", e)),
        }
    }

    // --------------------------------------------------
    // Sync
    // --------------------------------------------------

    println!("🔄 Syncing...");

    let sync_command = build_command(10);
    let sync_packet = encrypt_command(&sync_command);

    send_command(
        &handle,
        &sync_packet,
        Duration::from_millis(2000),
    )?;

    std::thread::sleep(Duration::from_millis(200));

    // --------------------------------------------------
    // Create image
    // --------------------------------------------------

    println!("🎨 Rendering text...");

    let mut image: RgbaImage = ImageBuffer::from_pixel(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        Rgba([0, 0, 0, 255]),
    );

    // Change this path to your actual font.
    let font = FontRef::try_from_slice(
        include_bytes!("../assets/dejavu-sans-bold.ttf")
    )
    .map_err(|_| "Failed to load font".to_string())?;

    let font_size = 100.0;

    let scale = PxScale::from(font_size);

    // --------------------------------------------------
    // Basic text rendering
    // --------------------------------------------------

    let words: Vec<&str> = message.split_whitespace().collect();

    let mut lines = Vec::new();
    let mut current = String::new();

    let max_width = IMAGE_WIDTH - 40;

    for word in words {
        let test = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };

        let width = imageproc::drawing::text_size(
            scale,
            &font,
            &test,
        ).0;

        if width <= max_width {
            current = test;
        } else {
            if !current.is_empty() {
                lines.push(current);
            }

            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    let line_height = font_size as i32 + 20;

    let total_height = lines.len() as i32 * line_height;

    let mut y = (IMAGE_HEIGHT as i32 - total_height) / 2;

    for line in lines {
        let (text_width, _) =
            imageproc::drawing::text_size(
                scale,
                &font,
                &line,
            );

        let x =
            (IMAGE_WIDTH as i32 - text_width as i32) / 2;

        draw_text_mut(
            &mut image,
            Rgba([255, 255, 255, 255]),
            x,
            y,
            scale,
            &font,
            &line,
        );

        y += line_height;
    }

    // --------------------------------------------------
    // Encode PNG
    // --------------------------------------------------

    println!("🖼️ Encoding PNG...");

    let mut png_data = Vec::new();

    {
        let mut cursor =
            std::io::Cursor::new(&mut png_data);

        image
            .write_to(
                &mut cursor,
                image::ImageFormat::Png,
            )
            .map_err(|e| format!("PNG encoding failed: {}", e))?;
    }

    println!(
        "📦 PNG size: {} bytes",
        png_data.len()
    );

    if png_data.len() > 1_048_576 {
        return Err(
            "PNG is larger than 1MB".to_string()
        );
    }

    // --------------------------------------------------
    // Command 102
    // --------------------------------------------------

    let size = png_data.len();

    let mut command = build_command(102);

    command[8] = ((size >> 24) & 0xFF) as u8;
    command[9] = ((size >> 16) & 0xFF) as u8;
    command[10] = ((size >> 8) & 0xFF) as u8;
    command[11] = (size & 0xFF) as u8;

    let encrypted_command =
        encrypt_command(&command);

    // --------------------------------------------------
    // Combine encrypted command + PNG
    // --------------------------------------------------

    let mut payload =
        Vec::with_capacity(
            encrypted_command.len() + png_data.len()
        );

    payload.extend_from_slice(
        &encrypted_command
    );

    payload.extend_from_slice(
        &png_data
    );

    println!(
        "📤 Sending {} bytes...",
        payload.len()
    );

    // --------------------------------------------------
    // Send to TURZX
    // --------------------------------------------------

    let response = send_command(
        &handle,
        &payload,
        Duration::from_millis(5000),
    )?;

    match response {
        Some(_) => {
            println!("✅ Image sent to TURZX1.0");
        }

        None => {
            println!(
                "⚠️ Image sent, but no response"
            );
        }
    }

    Ok(format!(
        "Displayed: {}",
        message
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![
                send_message
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}