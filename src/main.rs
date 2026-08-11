mod camera;
mod config;
mod preview;
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

    let select_camera_desc = cameras_desc.next();

    let camera = camera::Camera::open()?;
    // camera.print_summary();
    Ok(camera)
}

fn create_ui(camera: &Arc<camera::Camera>) -> Result<Arc<MainWindow>, slint::PlatformError> {
    let main_window = Arc::new(MainWindow::new()?);
    main_window.global::<CameraControl>().on_take_photo({
        let camera = camera.clone();
        move || {
            camera.take_photo();
        }
    });

    Ok(main_window)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = config::Config::load()?;

    let camera = Arc::new(open_camera()?);
    let main_window = create_ui(&camera)?;

    let weak_window = main_window.as_weak();
    let update_preview: preview::PreviewCallback = Arc::new(move |pixel_buffer| {
        let weak_window = weak_window.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(window) = weak_window.upgrade() {
                let image = Image::from_rgb8(pixel_buffer);
                window.set_preview_image(image);
            }
        }).unwrap();
    });

    let preview_thread = preview::PreviewThread::new(&camera, &update_preview);

    main_window.run()?;

    println!("exit...");

    Ok(())
}
