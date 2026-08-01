use std::print;
use std::error::Error;
use std::result::Result;

use gphoto2::Camera as GCamera;
use gphoto2::file::CameraFile;
use gphoto2::filesys::CameraFS;
use gphoto2::list::CameraListIter;
use gphoto2::task::Task;
use gphoto2::{Context};
use slint::{Image, SharedPixelBuffer};

pub struct Camera {
    context: Context,
    camera: GCamera,
    // camera_fs: CameraFS<'a>,
}

impl Camera {
    pub fn open() -> gphoto2::Result<Camera> {
        let context = Context::new()?;
        let camera = context
            .autodetect_camera()
            .wait()
            .expect("Failed to autodetect camera");
        // let camera_fs = camera.fs();
        Ok(Camera {
            context,
            camera,
            // camera_fs,
        })
    }

    pub fn list_cameras_desc() -> gphoto2::Result<CameraListIter> {
        Context::new()?.list_cameras().wait()
    }

    pub fn print_summary(&self) {
        match self.camera.summary() {
            Ok(summary) => println!("{}", summary),
            Err(_) => println!("Faile to retrieve summary"),
        }
    }

    pub async fn capture_preview(&self) -> Result<Image, Box<dyn Error>> {
        let file = self.camera.capture_preview().await?;
        let content_bytes = file.get_data(&self.context).await?;
        let decoded_image = image::load_from_memory(&content_bytes)?.into_rgb8();
        let shared_buffer = SharedPixelBuffer::clone_from_slice(
            decoded_image.as_raw(),
            decoded_image.width(),
            decoded_image.height(),
        );
        Ok(Image::from_rgba8(shared_buffer))
    }

    pub fn take_photo(&self) {

    }
}
