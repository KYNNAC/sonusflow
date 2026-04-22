use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, StreamConfig};
use slint::{ComponentHandle, Model};

mod audio;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize audio input
    let _audio_stream = audio::setup_audio()?;

    // Initialize UI
    let ui = ui::MainWindow::new();

    // Start UI event loop
    ui.run()?;

    Ok(())
}