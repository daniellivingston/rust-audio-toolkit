/*
 * Based on the Aubio example program `aubionotes`:
 *  <https://aubio.org/manual/latest/cli.html#aubionotes>
 *  <https://github.com/aubio/aubio/blob/master/examples/aubionotes.c>
 */

use hound::{WavReader, Error};
use aubio::Notes;
use aubio::{Pitch, PitchMode};

fn norm(samples: &mut [f32]) {
    for i in 0..samples.len() {
        samples[i] = samples[i].abs();
    }
}

fn test_output() {
    let data_str = include_str!("data/c3-major-scale-piano.freq.txt");
    let data = data_str
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|s| {
            println!("{}", s)
        });
}

fn process(samples: Vec<f32>, spec: &hound::WavSpec) {
    let buff_size = 2048;
    let hop_size = 256;
    let tolerance = 0.0;
    let silence = -90.0;
    let pitch_unit = aubio::PitchUnit::Hz;

    println!("Performing pitch detection: {:#?}", spec);
    println!("  samples: {}", samples.len());
    println!("  sample rate: {}", spec.sample_rate);
    println!("  total time: {} s", samples.len() as f32 / spec.sample_rate as f32);

    let mut pitch = Pitch::new(
            PitchMode::Yinfft,
            buff_size,
            hop_size,
            spec.sample_rate
        ).unwrap()
        //.with_silence(silence)
        //.with_tolerance(tolerance)
        .with_unit(pitch_unit);

    let mut output = vec![0.0; samples.len() / hop_size];
    for (i, sample) in samples.chunks_exact(hop_size).enumerate() {
        pitch.do_(&sample, &mut output[i..=i]).unwrap();

    }

    let eps = 1e-5;
    let sum = output.iter().fold(0.0, |acc, x| acc + x);
    assert!(sum > eps, "empty output - frequency extraction failed");

    // print the results
    for i in 0..(output.len() as usize) {
        let t = i as f32 * (hop_size as f32 / spec.sample_rate as f32);
        let x = output[i];
        println!("{} {}", t, x);
    }
}

fn read_wav_file(path: &str) -> Result<(), Error> {
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();
    let samples: Vec<i32> = reader.samples().map(|s| s.unwrap()).collect();

    let mut s2: Vec<f32> = samples.iter().map(|s| *s as f32).collect();

    // Normalize
    // norm(s2.as_mut_slice());

    println!("{} vs. {}", samples[0], s2[0]);

    process(s2, &spec);

    Ok(())
}

fn main() {
    read_wav_file("examples/data/c3-major-scale-piano.wav").unwrap();
}
