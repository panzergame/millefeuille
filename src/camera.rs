use std::error::Error;
use std::result::Result;
use std::time::Instant;

use gphoto2::Camera as GCamera;
use gphoto2::Context;
use gphoto2::file::CameraFile;
use gphoto2::filesys::CameraFS;
use gphoto2::list::CameraListIter;
use gphoto2::task::Task;
use gphoto2::widget::Widget::Radio;
use image::EncodableLayout;
use slint::{Rgb8Pixel, SharedPixelBuffer};
use turbojpeg::{decompress, PixelFormat};


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
            .wait()?;
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

    pub fn capture_preview(&self) -> Result<SharedPixelBuffer<Rgb8Pixel>, Box<dyn Error>> {
        let file = self.camera.capture_preview().wait()?;
        let content_bytes = file.get_data(&self.context).wait()?;
        let image = decompress(
            &content_bytes,
            PixelFormat::RGB,
        )?;

        let shared_buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
            image.pixels.as_bytes(),
            image.width as u32,
            image.height as u32,
        );

        Ok(shared_buffer)
    }

    pub fn take_photo(&self) {}

    async fn radio_property_choices(&self, name: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let widget = self.camera.config_key(name).await?;
        match widget {
            Radio(widget) => Ok(vec![]),
            _ => Err("not a radio property".into()),
        }
    }
}
