use rasciigraph; // temporary

use cpal::{
    platform::HostId,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use log::info;
use pitch_detection::{float::Float, Pitch};
use std::sync::{Arc, Mutex};
use hound;

#[allow(dead_code)]
fn graph(data: &Vec<f64>, caption: String) {
    println!("{}",
        rasciigraph::plot(
            data.clone(),
            rasciigraph::Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption(caption)
        )
    );
}

#[allow(unused)]
#[derive(Debug)]
pub struct Audio<T> {
    raw_data: Vec<T>,
    duration: std::time::Duration,
    sample_rate: cpal::SampleRate,
    channels: cpal::ChannelCount,
}

impl<T> Audio<T> {
    pub fn new(
        data: Vec<T>,
        duration: std::time::Duration,
        sample_rate: cpal::SampleRate,
        channels: cpal::ChannelCount,
    ) -> Audio<T> {
        Audio {
            raw_data: data,
            duration: duration,
            sample_rate: sample_rate,
            channels: channels,
        }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.raw_data
    }

    pub fn duration(&self) -> &std::time::Duration {
        &self.duration
    }
    /*
        let spec = reader.spec();
        let duration = std::time::Duration::from_secs((reader.duration() / spec.sample_rate) as u64).into();
        let sample_rate = cpal::SampleRate(spec.sample_rate);
        let channels = spec.channels as cpal::ChannelCount;

        let samples: Vec<f32> = reader.samples::<i16>() // WARNING: i16 is ONLY valid for c3-major-scale-piano.wav
                                      .map(|sample| sample.unwrap() as f32 / std::i16::MAX as f32)
                                      .collect();

        Ok(Audio::new(samples, duration, sample_rate, channels))
        */

    ///////////////////////////////////////////////////
    // pub fn fft(&self) -> Result<Vec<f64>> {
    // --> https://siciarz.net/24-days-rust-hound/

    pub fn from_freq(freq_hz: f64, duration: f64) -> Result<Audio<i32>, anyhow::Error> {
        use std::f64::consts::PI;

        let sample_rate = cpal::SampleRate(44100);
        let channels = 1;
        let duration = std::time::Duration::from_secs_f64(duration);

        let amp = 10_000.0; // amplitude

        let samples: Vec<i32> =
            (0..(sample_rate.0 as f64 * duration.as_secs_f64()) as i32)
                .map(|i| {
                    let t = (i / sample_rate.0 as i32) as f64;
                    (amp * (2.0 * PI * freq_hz * t).sin()) as i32
                })
                .collect();

        Ok(Audio::new(samples, duration, sample_rate, channels))
    }

    pub fn from_wav(path: &String) -> Result<Audio<i32>, anyhow::Error> {
        let mut reader: hound::WavReader<_> = hound::WavReader::open(path)?;
        let spec = reader.spec();

        println!("spec: {:?}", spec);

        match (spec.bits_per_sample, spec.sample_format) {
            (16, hound::SampleFormat::Int) |
            (32, hound::SampleFormat::Int) => {
                let samples = reader
                                            .samples::<i16>()
                                            .map(|x| x.unwrap() as i32)
                                            .collect();

                Ok(Audio::new(
                    samples,
                    std::time::Duration::from_secs((reader.duration() / spec.sample_rate) as u64).into(),
                    cpal::SampleRate(spec.sample_rate),
                    spec.channels as cpal::ChannelCount,
                ))
            }
            _ => {
                unreachable!()
            }
        }
    }

}

pub fn system_overview() {
    let hosts: Vec<HostId> = cpal::platform::available_hosts();
    println!("Available hosts (count = {}):", hosts.len());
    for host in hosts {
        println!("- {}", host.name());
    }

    let default_host = cpal::default_host();
    println!("\nUsing default host: {}", default_host.id().name());

    let input_devices: Vec<cpal::Device> = default_host.input_devices().unwrap().collect();
    println!(
        "\nAvailable input devices (count = {}):",
        input_devices.len()
    );
    for device in input_devices {
        println!(
            "- {}",
            device.name().unwrap_or(String::from("ERROR_UNKNOWN"))
        );
    }

    let output_devices: Vec<cpal::Device> = default_host.output_devices().unwrap().collect();
    println!(
        "\nAvailable output devices (count = {}):",
        output_devices.len()
    );
    for device in output_devices {
        println!(
            "- {}",
            device.name().unwrap_or(String::from("ERROR_UNKNOWN"))
        );
    }
}

fn capture_input(duration: std::time::Duration) -> Result<Audio<f32>, anyhow::Error> {
    let device = cpal::default_host()
        .default_input_device()
        .expect("no input device available");
    info!("Input device: {}", device.name()?);

    let config = device
        .default_input_config()
        .expect("failed to get default input config");

    let buffer: Vec<f32> = vec![];
    let buffer = Arc::new(Mutex::new(Some(buffer)));

    info!("Begin recording...");

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
                data_fn(data, &buffer_2);
            },
            move |err: cpal::StreamError| eprintln!("An error occurred on stream: {}", err),
        )?,
        sample_format => {
            panic!("Unsupported sample format: {:#?}", sample_format);
        }
    };

    stream.play()?;

    // Let recording run for the specified duration.
    std::thread::sleep(duration);
    drop(stream);
    info!("Finished recording");

    let buffer = buffer
        .lock()
        .expect("Could not take lock on recording buffer")
        .take()
        .expect("Could not reclaim mutex on recording buffer");

    Ok(Audio::new(buffer, duration, sample_rate, channels))
}

fn play_buffer(audio: Audio<f32>) -> Result<(), anyhow::Error> {
    let device = cpal::default_host()
        .default_output_device()
        .expect("no output device available");
    println!("Output device: {}", device.name()?);

    let config = device
        .default_output_config()
        .expect("failed to get default output config");

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
        } else {
            last
        }
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                data.iter_mut().for_each(|d| *d = next_value())
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
        false => signal.len(),
    };

    let stop = match signal.len() >= start + window {
        true => start + window,
        false => signal.len(),
    };

    for i in 0..stop - start {
        output[i] = signal[start + i];
    }

    for i in stop - start..output.len() {
        output[i] = T::zero();
    }
}

fn get_pitches(audio: &Audio<f32>) -> Vec<Pitch<f32>> {
    use pitch_detection::detector::mcleod::McLeodDetector;
    use pitch_detection::detector::PitchDetector;
    use pitch_detection::utils::buffer::new_real_buffer;

    const POWER_THRESHOLD: f32 = 0.0;
    const CLARITY_THRESHOLD: f32 = 0.8;
    const WINDOW: usize = 1024;
    const PADDING: usize = WINDOW / 2;
    const DELTA_T: usize = WINDOW / 4;

    let mut detector = McLeodDetector::new(WINDOW, PADDING);
    let mut chunk = new_real_buffer(WINDOW);

    let sample_rate = audio.sample_rate.0 as usize;
    let duration: f32 = audio.raw_data.len() as f32 / sample_rate as f32;
    let sample_size: usize = (audio.sample_rate.0 as f32 * duration) as usize;
    let n_windows: usize = (sample_size - WINDOW) / DELTA_T;

    (0..n_windows).filter_map(|i| {
        get_chunk(&audio.raw_data, i * DELTA_T, WINDOW, &mut chunk);
        detector.get_pitch(&chunk, sample_rate, POWER_THRESHOLD, CLARITY_THRESHOLD)
    })
    .collect()
}

fn print_detected_pitches(audio: &Audio<f32>) -> Result<(), anyhow::Error> {
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

        let pitch = detector.get_pitch(&chunk, sample_rate, POWER_THRESHOLD, CLARITY_THRESHOLD);

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
                println!("(no pitch detected)");
            }
        }
    }

    Ok(())
}

pub fn read_wav(path: &String) -> Result<Audio<f32>, anyhow::Error> {
    let mut reader = hound::WavReader::open(path)?;

    let spec = reader.spec();
    let duration = std::time::Duration::from_secs((reader.duration() / spec.sample_rate) as u64).into();
    let sample_rate = cpal::SampleRate(spec.sample_rate);
    let channels = spec.channels as cpal::ChannelCount;

    let samples: Vec<f32> = reader.samples::<i16>() // WARNING: i16 is ONLY valid for c3-major-scale-piano.wav
                                  .map(|sample| sample.unwrap() as f32 / std::i16::MAX as f32)
                                  .collect();

    Ok(Audio::new(samples, duration, sample_rate, channels))
}

pub fn analyze_wav(path: std::path::PathBuf) -> Result<(), anyhow::Error> {
    let mut reader = hound::WavReader::open(&path)?;
    let spec = reader.spec();

    println!("Path: {:?}", path);
    println!("Duration: {}s", reader.duration() / spec.sample_rate);
    println!("{:#?}", spec);
    println!("------------");

    let duration = std::time::Duration::from_secs((reader.duration() / spec.sample_rate) as u64).into();
    let sample_rate = cpal::SampleRate(spec.sample_rate);
    let channels = spec.channels as cpal::ChannelCount;
    let samples: Vec<f32> = reader
                              .samples::<i16>() // WARNING: i16 is ONLY valid for c3-major-scale-piano.wav
                              //.step_by(spec.channels as usize)
                              .map(|x| x.unwrap() as f32)
                              .collect::<Vec<_>>();

    // {
    //     let samples: Vec<f64> = samples.iter().map(|&x| x as f64).step_by(5000).collect();
    //     graph(&samples, "Power Spectrum".to_string());
    // }

    let audio = Audio::new(samples, duration, sample_rate, channels);
    let pitches = get_pitches(&audio);

    println!("Detected {} pitches", pitches.len());
    let mut pitch_min: f32 = 1e7;
    let mut pitch_max: f32 = -1e7;
    //let mut pitch_set = HashSet::new();
    for pitch in pitches {
        if pitch.frequency < pitch_min {
            pitch_min = pitch.frequency;
        }
        if pitch.frequency > pitch_max {
            pitch_max = pitch.frequency;
        }
    }
    println!("Min = {}; Max = {}", pitch_min, pitch_max);

    // {
    //     let freqs: Vec<f64> = pitches.iter().map(|x| x.frequency as f64).step_by(100).collect();
    //     graph(&freqs, "Detected Frequencies".to_string());
    // }

    Ok(())
}

pub fn system_test() -> Result<(), anyhow::Error> {
    let capture_duration = std::time::Duration::from_secs(3);
    println!("Recording for duration: {:#?}", capture_duration);

    let captured = capture_input(capture_duration)?;
    println!("Captured: {}", captured.raw_data.len());

    println!("Analyzing...");
    let _ = print_detected_pitches(&captured);

    println!("Playing back...");
    play_buffer(captured)?;
    println!("Finished playback.");

    Ok(())
}
