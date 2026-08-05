mod camera;
mod config;
use std::{error::Error, sync::Arc};
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
    // camera.print_summary();
    Ok(camera)
}

async fn capture_preview_task(camera: Arc<camera::Camera>, window: slint::Weak<MainWindow>) {
    match camera.capture_preview().await {
        Ok(image) => {
            if let Some(window) = window.upgrade() {
                window.set_preview_image(image);
            }
        }
        Err(error) => eprintln!("failed to get preview image: {error}"),
    }
}

fn start_preview_loop(camera: &Arc<camera::Camera>, main_window: &MainWindow) -> Timer {
    let timer: Timer = Timer::default();
    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(300),
        {
            let camera = camera.clone();
            let window = main_window.as_weak();
            move || {
                let camera = camera.clone();
                let window = window.clone();
                slint::spawn_local(async move {
                    capture_preview_task(camera, window).await;
                })
                .unwrap();
            }
        }
    );

    timer
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
    config.set_iso(config.iso() + 10);
    println!("iso: {}", config.iso());
    
    let camera = Arc::new(open_camera()?);
    let main_window = create_ui(&camera)?;
    let preview_timer = start_preview_loop(&camera, main_window.as_ref());

    main_window.run()?;
    Ok(())
}
