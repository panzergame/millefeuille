use std::print;

use gphoto2::Camera as GCamera;
use gphoto2::file::CameraFile;
use gphoto2::filesys::CameraFS;
use gphoto2::list::CameraListIter;
use gphoto2::task::Task;
use gphoto2::{Context, Result};

pub struct Camera {
    camera: GCamera,
    // camera_fs: CameraFS<'a>,
}

impl Camera {
    pub fn open() -> Result<Camera> {
        let camera = Context::new()?
            .autodetect_camera()
            .wait()
            .expect("Failed to autodetect camera");
        // let camera_fs = camera.fs();
        Ok(Camera {
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
            Err(_) => println!("Faile to retrieve summary")
        }
    }

    pub async fn capture_preview(&self) -> Result<CameraFile> {
        let image = self.camera.capture_preview().await;
        image
    } 
}
