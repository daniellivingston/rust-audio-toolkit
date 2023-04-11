/*
 * Based on the Aubio example program `aubionotes`:
 *  <https://aubio.org/manual/latest/cli.html#aubionotes>
 *  <https://github.com/aubio/aubio/blob/master/examples/aubionotes.c>
 */

use hound::{WavReader, Error};
use aubio::Notes;
use aubio::{Pitch, PitchMode};

fn process(samples: Vec<f32>, spec: &hound::WavSpec) {
    let buff_size = 2048;
    let hop_size = 256;

    let mut pitch = Pitch::new(PitchMode::Yinfft, buff_size, hop_size, spec.sample_rate).unwrap();
    let mut input = vec![0.0; hop_size];
    let mut output = vec![0.0; 1];

    for (i, sample) in samples.chunks_exact(hop_size).enumerate() {
        input.copy_from_slice(sample);
        pitch.do_(input.as_slice(), output.as_mut_slice()).unwrap();
        println!("{}: {:?}", i, output);
    }
}

fn read_wav_file(path: &str) -> Result<(), Error> {
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    let s2: Vec<f32> = samples.iter().map(|s| *s as f32).collect();

    process(s2, &spec);

    Ok(())
}

fn main() {
    read_wav_file("examples/data/C_Major.wav").unwrap();
}
