use slint::{Image, Rgb8Pixel, SharedPixelBuffer};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::{thread, time};

use crate::camera::Camera;

const MIN_PREVIEW_INTERVAL: time::Duration = time::Duration::from_millis(40);

pub type PreviewCallback = Arc<dyn Fn(SharedPixelBuffer<Rgb8Pixel>) -> () + Send + Sync>;

struct PreviewManager {
    camera: Arc<Camera>,
    callback: PreviewCallback,
    running: AtomicBool,
}

pub struct PreviewThread {
    manager: Arc<PreviewManager>,
    thread: Option<JoinHandle<()>>,
}

impl PreviewThread {
    pub fn new(camera: &Arc<Camera>, callback: &PreviewCallback) -> PreviewThread {
        let manager = Arc::new(PreviewManager {
            camera: camera.clone(),
            callback: callback.clone(),
            running: AtomicBool::new(true),
        });

        let thread = std::thread::spawn({
            let manager = manager.clone();
            move || {
                manager.preview_loop();
            }
        });

        PreviewThread {
            manager,
            thread: Some(thread),
        }
    }
}

impl Drop for PreviewThread {
    fn drop(&mut self) {
        self.manager.running.store(false, Ordering::Relaxed);
        self.thread
            .take()
            .unwrap()
            .join()
            .expect("Failed terminating preview thread");
    }
}

impl PreviewManager {
    fn preview_loop(&self) {
        while self.running.load(Ordering::Relaxed) {
            match self.camera.capture_preview() {
                Ok(pixel_buffer) => {
                    (self.callback)(pixel_buffer);
                }
                Err(error) => eprintln!("failed to get preview image: {error}"),
            }
            thread::sleep(MIN_PREVIEW_INTERVAL);
        }
    }
}
