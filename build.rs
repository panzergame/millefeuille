fn main() {
    slint_build::compile("ui/camera-control.slint").expect("Slint build failed");
    slint_build::compile("ui/main-window.slint").expect("Slint build failed");
}