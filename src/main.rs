slint::include_modules!();

mod audio; 

fn main() -> Result<(), slint:PlatformError> {
    let ui = AppWindow::new()?;

    // Audio engine in a separate thread
    std::thread::spawn(|| {
        audio::start_audio_engine().expect("Failed to start audio engine");
    });

    ui.run()
}