mod camera;
mod config;
use std::{error::Error, print, println, sync::Arc};
use slint::{Timer, TimerMode};

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
    camera.print_summary();
    Ok(camera)
}

fn start_preview_loop(camera: &Arc<camera::Camera>) -> Timer {
    let timer = Timer::default();
    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(100),
        {
            let camera = camera.clone();
            move || {
                let camera = camera.clone();
                slint::spawn_local({
                    async move {
                        let image = camera.capture_preview().await;
                        match image {
                            Ok(_) => println!("got image"),
                            Err(_) => println!("failed to get image"),
                        }
                    }
                })
                .unwrap();
            }
        },
    );

    timer
}

fn create_ui(camera: &Arc<camera::Camera>) -> Result<MainWindow, slint::PlatformError> {
    let main_window = MainWindow::new()?;
    main_window.global::<CameraControl>().on_take_photo({
        let camera = camera.clone();
        move || {
            camera.take_photo();
        }
    });

    Ok(main_window)
}

fn main() -> Result<(), Box<dyn Error>> {
    // let camera = Arc::new(open_camera()?);
    // let _ = start_preview_loop(&camera);

    let mut config = config::Config::load()?;
    config.set_iso(config.iso() + 10);
    println!("iso: {}", config.iso());
    
    // let main_window = create_ui(&camera)?;
    // main_window.run()?;
    Ok(())
}
