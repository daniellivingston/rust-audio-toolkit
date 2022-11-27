use std::sync::{Arc, Mutex};
use cpal::{
    platform::HostId,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

fn system_overview() {
    let hosts: Vec<HostId> = cpal::platform::available_hosts();
    println!("Available hosts (count = {}):", hosts.len());
    for host in hosts {
        println!("- {}", host.name());
    }

    let default_host = cpal::default_host();
    println!("\nUsing default host: {}", default_host.id().name());

    let input_devices: Vec<cpal::Device> = default_host.input_devices().unwrap().collect();
    println!("\nAvailable input devices (count = {}):", input_devices.len());
    for device in input_devices {
        println!("- {}", device.name().unwrap_or(String::from("ERROR_UNKNOWN")));
    }

    let output_devices: Vec<cpal::Device> = default_host.output_devices().unwrap().collect();
    println!("\nAvailable output devices (count = {}):", output_devices.len());
    for device in output_devices {
        println!("- {}", device.name().unwrap_or(String::from("ERROR_UNKNOWN")));
    }
}

fn capture_input(duration: std::time::Duration) -> Result<Vec<f32>, anyhow::Error> {
    let device = cpal::default_host()
        .default_input_device()
        .expect("no input device available");
    println!("Input device: {}", device.name()?);

    let config = device
        .default_input_config()
        .expect("failed to get default input config");
    println!("Default input config: {:?}", config);

    let buffer: Vec<f32> = vec![];
    let buffer = Arc::new(Mutex::new(Some(buffer)));

    println!("Begin recording...");

    type ArcLockedVec = Arc<Mutex<Option<Vec<f32>>>>;
    let data_fn = move |data: &[f32], buffer: &ArcLockedVec| {
        if let Ok(mut guard) = buffer.try_lock() {
            if let Some(buffer) = guard.as_mut() {
                buffer.append(&mut data.to_vec());
            }
        }
    };

    let buffer_2 = buffer.clone();
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                data_fn(data, &buffer_2);
            },
            move |err: cpal::StreamError| eprintln!("An error occurred on stream: {}", err),
        )?,
        sample_format => {
            panic!("Unsupported sample format: {:?}", sample_format);
        }
    };

    stream.play()?;

    // Let recording run for the specified duration.
    std::thread::sleep(duration);
    drop(stream);
    let x = buffer
        .lock()
        .expect("Could not take lock on recording buffer")
        .take()
        .expect("Could not reclaim mutex on recording buffer");

    println!("Finished recording");

    Ok(x)
}

fn main() -> Result<(), anyhow::Error> {
    println!("\n\nDEVICE & DRIVER OVERVIEW\n");
    system_overview();

    let input_data = capture_input(std::time::Duration::from_secs(1))?;
    println!("Captured: {:?}", input_data.len());
    println!("Debug: {:?}", input_data);

    Ok(())
}
