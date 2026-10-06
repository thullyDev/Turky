use std::io::Cursor;
use std::time::Duration;

use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, Frame, RgbaImage};

const ZERO_FRAME_DELAY: Duration = Duration::from_millis(100);

#[derive(Debug)]
pub struct GifFrame {
    pub image: RgbaImage,
    pub delay: Duration,
}

pub struct GifCutter;

impl GifCutter {
    pub fn new() -> Self {
        Self
    }

    pub fn cut(&self, bytes: &[u8]) -> Result<Vec<GifFrame>, String> {
        let decoder = GifDecoder::new(Cursor::new(bytes))
            .map_err(|error| format!("Failed to create GIF decoder: {error}"))?;

        let mut frames = decoder.into_frames();
        let mut decoded = Vec::new();

        loop {
            let Some(frame_result) = frames.next() else {
                break;
            };

            let frame =
                frame_result.map_err(|error| format!("Failed to decode GIF frame: {error}"))?;
            let delay = self.delay_of(&frame);

            decoded.push(GifFrame {
                image: frame.into_buffer(),
                delay,
            });
        }

        Ok(decoded)
    }

    fn delay_of(&self, frame: &Frame) -> Duration {
        let (numer, denom) = frame.delay().numer_denom_ms();

        if denom == 0 {
            return ZERO_FRAME_DELAY;
        }

        let millis = u64::from(numer) / u64::from(denom);

        if millis == 0 {
            return ZERO_FRAME_DELAY;
        }

        Duration::from_millis(millis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gif::{Encoder, Frame, Repeat};
    use std::time::Duration;

    fn two_frame_gif() -> Vec<u8> {
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
    fn cuts_a_gif_into_frames() {
        let frames = GifCutter::new()
            .cut(&two_frame_gif())
            .expect("gif should cut");

        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].image.width(), 2);
        assert_eq!(frames[0].image.height(), 2);
        assert_eq!(frames[0].delay, Duration::from_millis(10));
        assert_eq!(frames[1].image.as_raw().len(), 2 * 2 * 4);
    }

    #[test]
    fn treats_a_zero_delay_as_one_hundred_milliseconds() {
        let mut bytes = Vec::new();

        {
            let mut encoder =
                Encoder::new(&mut bytes, 1, 1, &[]).expect("Failed to create GIF encoder");

            encoder
                .set_repeat(Repeat::Infinite)
                .expect("Failed to set GIF repeat");

            let mut frame = Frame::from_rgb(1, 1, &[0, 0, 0]);

            frame.delay = 0;

            encoder
                .write_frame(&frame)
                .expect("Failed to write GIF frame");
        }

        let frames = GifCutter::new().cut(&bytes).expect("gif should cut");

        assert_eq!(frames[0].delay, Duration::from_millis(100));
    }

    #[test]
    fn rejects_bytes_that_are_not_a_gif() {
        let error = GifCutter::new()
            .cut(b"this is not a gif")
            .expect_err("invalid gif should fail");

        assert!(error.contains("Failed to create GIF decoder"));
    }
}
