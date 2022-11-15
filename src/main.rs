use cpal::platform::available_hosts;
use cpal::traits::{HostTrait, DeviceTrait};

fn main() {
    println!("Hello, world!");

    for host in available_hosts() {
        println!("Found host: {:#?}", host.name());
    }

    let host = cpal::default_host();

    let names: Vec<String> = host.input_devices().unwrap().map(|x| x.name().unwrap()).collect();
    println!("Found devices: {:#?}", names);

    let device = host.default_output_device()
                     .expect("no output device available");

    println!("Initialized device '{:#?}'", device.name());
}
