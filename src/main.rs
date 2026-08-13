mod camera;
mod config;
mod preview;
use image::open;
use slint::Image;
use std::{error::Error, sync::Arc};

slint::include_modules!();

fn open_camera() -> Result<camera::Camera, Box<dyn Error>> {
    let mut cameras_desc = camera::Camera::list_cameras_desc()?;
    let cameras_desc = cameras_desc.by_ref();
    println!("Detected models:");
    cameras_desc.for_each(|camera_desc| {
        println!("\t{}", camera_desc.model);
    });

    let camera = camera::Camera::open()?;
    Ok(camera)
}

fn show_main_screen(
    main_window: &Arc<MainWindow>,
    camera: Arc<camera::Camera>,
) -> Result<(), slint::PlatformError> {
    main_window.set_failed_open_camera(false);

    main_window.global::<CameraControl>().on_take_photo({
        let camera = camera.clone();
        move || {
            camera.take_photo();
        }
    });

    let weak_window = main_window.as_weak();
    let update_preview: preview::PreviewCallback = Arc::new(move |pixel_buffer| {
        let weak_window = weak_window.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(window) = weak_window.upgrade() {
                let image = Image::from_rgb8(pixel_buffer);
                window.set_preview_image(image);
            }
        })
        .unwrap();
    });

    let preview_thread = preview::PreviewThread::new(&camera, &update_preview);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = config::Config::load()?;

    let main_window = Arc::new(MainWindow::new()?);
    main_window.on_retry_open_camera({
        let main_window = main_window.clone();
        move || {
            if let Ok(camera) = open_camera() {
                let _ = show_main_screen(&main_window, Arc::new(camera));
            }
        }
    });

    match open_camera() {
        Ok(camera) => show_main_screen(&main_window, Arc::new(camera))?,
        Err(_) => {
            main_window.set_failed_open_camera(true);
        }
    }
    main_window.run()?;

    println!("exit...");

    Ok(())
}
