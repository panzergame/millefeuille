mod camera;
use std::{error::Error, print, println};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let mut cameras_desc = camera::Camera::list_cameras_desc()?;
    let cameras_desc = cameras_desc.by_ref();
    println!("Detected models:");
    cameras_desc.for_each(|camera_desc|{
        println!("\t{}", camera_desc.model);
    });

    let select_camera_desc = cameras_desc.next();

    //let camera = camera::Camera::open()?;
    // camera.print_summary();

    let main_window = MainWindow::new()?;
    main_window.global::<CameraControl>().on_take_photo(||{
        println!("take photo");
    });

    main_window.run()?;
    Ok(())
}
