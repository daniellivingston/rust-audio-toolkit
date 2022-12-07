use std::sync::{Arc, Mutex};
use cpal::{
    platform::HostId,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use pitch_detection::float::Float;

#[allow(unused)]
#[derive(Debug)]
pub struct Audio {
    raw_data: Vec<f32>,
    duration: std::time::Duration,
    sample_rate: cpal::SampleRate,
    channels: cpal::ChannelCount
}

impl Audio {
    pub fn new(data: Vec<f32>,
               duration: std::time::Duration,
               sample_rate: cpal::SampleRate,
               channels: cpal::ChannelCount) -> Audio {
        Audio {
            raw_data: data,
            duration: duration,
            sample_rate: sample_rate,
            channels: channels
        }
    }
}

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

fn capture_input(duration: std::time::Duration) -> Result<Audio, anyhow::Error> {
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

    let sample_rate = config.sample_rate();
    let channels = config.channels();

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
                println!("data: {:?}", data);
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
    println!("Finished recording");

    let buffer = buffer
        .lock()
        .expect("Could not take lock on recording buffer")
        .take()
        .expect("Could not reclaim mutex on recording buffer");

    Ok(Audio::new(buffer, duration, sample_rate, channels))
}

fn play_buffer(audio: Audio) -> Result<(), anyhow::Error> {
    let device = cpal::default_host()
        .default_output_device()
        .expect("no output device available");
    println!("Output device: {}", device.name()?);

    let config = device
        .default_output_config()
        .expect("failed to get default output config");
    println!("Default output config: {:?}", config);

    // TODO: these complications come from trying to translate one channel (microphone)
    // into two (stereo speakers).
    // Here, we are artificially interleaving the mono channel twice to mimic stereo.
    let mut data = audio.raw_data.clone().into_iter();
    let mut flag = false;
    let mut last = 0f32;
    let mut next_value = move || -> f32 {
        flag = !flag;
        if flag {
            last = data.next().unwrap_or(0f32);
            last
        }
        else {
            last
        }
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                data.iter_mut()
                    .for_each(|d| *d = next_value())
            },
            move |err| eprintln!("an error occurred on stream: {}", err),
        )?,
        sample_format => {
            panic!("Unsupported sample format: {:?}", sample_format);
        }
    };
    stream.play()?;

    std::thread::sleep(audio.duration);
    drop(stream);

    Ok(())
}

fn get_chunk<T: Float>(signal: &[T], start: usize, window: usize, output: &mut [T]) {
    let start = match signal.len() > start {
        true => start,
        false => signal.len()
    };

    let stop = match signal.len() >= start + window {
        true => start + window,
        false => signal.len()
    };

    for i in 0..stop - start {
        output[i] = signal[start + i];
    }

    for i in stop - start..output.len() {
        output[i] = T::zero();
    }
}

fn print_detected_pitches(audio: &Audio) -> Result<(), anyhow::Error> {
    use pitch_detection::detector::mcleod::McLeodDetector;
    use pitch_detection::detector::PitchDetector;
    use pitch_detection::utils::buffer::new_real_buffer;

    let sample_rate = audio.sample_rate.0 as usize;
    let duration: f32 = audio.raw_data.len() as f32 / sample_rate as f32;
    let sample_size: usize = (audio.sample_rate.0 as f32 * duration) as usize;

    const WINDOW: usize = 1024;
    const PADDING: usize = WINDOW / 2;
    const DELTA_T: usize = WINDOW / 4;
    let n_windows: usize = (sample_size - WINDOW) / DELTA_T;

    const POWER_THRESHOLD: f32 = 0.0;
    const CLARITY_THRESHOLD: f32 = 0.6;

    let mut chunk = new_real_buffer(WINDOW);

    let mut detector = McLeodDetector::new(WINDOW, PADDING);

    for i in 0..n_windows {
        let t: usize = i * DELTA_T;

        get_chunk(&audio.raw_data, t, WINDOW, &mut chunk);

        let pitch = detector
            .get_pitch(&chunk, sample_rate, POWER_THRESHOLD, CLARITY_THRESHOLD);

        match pitch {
            Some(pitch) => {
                let frequency = pitch.frequency;
                let clarity = pitch.clarity;
                let idx = sample_rate as f32 / frequency;
                let epsilon = (sample_rate as f32 / (idx - 1.0)) - frequency;

                println!(
                    "[{idx}] Frequency: {:.2} +/- {:.2} Hz; Clarity: {:.2}",
                    frequency, epsilon, clarity,
                );
            }
            None => {
                eprintln!("Error: failed to detect pitch");
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    println!("\n\nDEVICE & DRIVER OVERVIEW\n");
    system_overview();

    let capture_duration = std::time::Duration::from_secs(3);
    let captured = capture_input(capture_duration)?;
    println!("Captured: {:?}", captured.raw_data.len());

    println!("Analyzing...");
    let _ = print_detected_pitches(&captured);

    println!("Playing back...");
    play_buffer(captured)?;
    println!("Finished playback.");

    Ok(())
}
