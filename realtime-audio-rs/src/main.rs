use cpal::traits::{HostTrait, DeviceTrait};
use cpal::platform::HostId;

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
}

fn main() {
    println!("\n\nDEVICE & DRIVER OVERVIEW\n");
    system_overview();

    println!("\n\nMICROPHONE SAMPLE DEMO\n");
    microphone_sample_demo();
}
