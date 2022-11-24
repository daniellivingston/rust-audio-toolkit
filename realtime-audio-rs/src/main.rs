use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample,
};
use cpal::platform::HostId;

// microphone -> speakers example: https://github.com/RustAudio/cpal/blob/master/examples/feedback.rs

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

fn microphone_sample_demo() {
    let host = cpal::default_host();

    let output_device = host.default_output_device()
                            .expect("no output device available");

    let input_device = host.default_input_device()
                            .expect("no input device available");

    println!("Demo configuration:");
    println!("  - Host:   '{}'", host.id().name());
    println!("  - Input:  '{}'", input_device.name().unwrap());
    println!("  - Output: '{}'", output_device.name().unwrap());

    let config = input_device
        .default_input_config()
        .expect("failed to get default input config");

    println!("Default input config: {:?}", config);

    let mut input_data: Vec<f32> = Vec::new();

    let data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        print!(".");
        &input_data.append(&mut data.to_owned());
    };

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => input_device.build_input_stream(
            &config.into(),
            data_fn,
            err_fn,
        ).expect("failed to build input stream"),
        sample_format => {
            panic!("Unsupported sample format: {:?}", sample_format);
        }
    };

    let record_duration: u64 = 3;

    print!("Recording for {} seconds.", record_duration);
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(record_duration));
    drop(stream);
    println!("Finished recording");
    // println!("Input data: {:?}", input_data);

    /////////////////////////////////////////////////////////////////////
    println!("Playing audio.");
    let output_config = output_device.default_output_config().unwrap();
    match output_config.sample_format() {
        cpal::SampleFormat::F32 => run(&output_device, &output_config.into()),
        _ => panic!("Unsupported sample format!"),
    }
    /////////////////////////////////////////////////////////////////////
    println!("Done playing audio.");
}

pub fn run(device: &cpal::Device, config: &cpal::StreamConfig)
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let value = next_value();
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
            //write_data(data, channels, &mut next_value)
        },
        err_fn,
    ).unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(3000));

}

// fn write_data(output: &mut [f32], channels: usize, next_sample: &mut dyn FnMut() -> f32)
// {
//     for frame in output.chunks_mut(channels) {
//         let value: T = T::from_sample(next_sample());
//         for sample in frame.iter_mut() {
//             *sample = value;
//         }
//     }
// }

fn main() {
    println!("\n\nDEVICE & DRIVER OVERVIEW\n");
    system_overview();

    println!("\n\nMICROPHONE SAMPLE DEMO\n");
    microphone_sample_demo();
}
