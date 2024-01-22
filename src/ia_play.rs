use bevy::ecs::prelude::{Entity, Resource};
use bevy::prelude::Image;
use bevy::utils::{HashMap, HashSet};
use image::ImageFormat;
use std::sync::Mutex;
use std::{path::Path, sync::PoisonError};
// fn save_gif(
//     path: &str,
//     frames: &mut Vec<Vec<u8>>,
//     speed: i32,
//     size: u16,
// ) -> Result<(), failure::Error> {
//     use gif::{Encoder, Frame, Repeat, SetParameter};
//
//     let mut image = std::fs::File::create(path)?;
//     let mut encoder = Encoder::new(&mut image, size, size, &[])?;
//     encoder.set(Repeat::Infinite)?;
//
//     for mut frame in frames {
//         encoder.write_frame(&Frame::from_rgba_speed(size, size, &mut frame, speed))?;
//     }
//
//     Ok(())
// }
//
//
/// A resource which allows for taking screenshots of the window.
pub type ScreenshotFn = Box<dyn FnOnce(Image) + Send + Sync>;

#[derive(Resource, Default)]
pub struct ScreenshotManager {
    // this is in a mutex to enable extraction with only an immutable reference
    pub(crate) callbacks: Mutex<HashSet<ScreenshotFn>>,
}

#[derive(Debug)]
pub struct ScreenshotAlreadyRequestedError;

fn count(x: impl FnOnce(u8)) {
    println!("count")
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {text}")
}

fn tee() {
    count(move |y| {
        let z = y + 1;
        println!("{z}");
    })
}

impl ScreenshotManager {
    pub fn take_screenshot(&mut self, callback: impl FnOnce(Image) + Send + Sync + 'static) {}

    /// Signals the renderer to take a screenshot of this frame.
    ///
    /// The screenshot will eventually be saved to the given path, and the format will be derived from the extension.
    pub fn save_screenshot_to_disk(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref().to_owned();
        self.take_screenshot(move |img| match img.try_into_dynamic() {
            Ok(dyn_img) => {
                let img = dyn_img.to_rgb8();
                img.save_with_format(&path, ImageFormat::Png).unwrap();
            }
            Err(e) => println!("Cannot save screenshot, screen format cannot be understood: {e}"),
        })
    }
}

fn main() {
    tee()
}
