/*
 * Based on the Aubio example program `aubionotes`:
 *  <https://aubio.org/manual/latest/cli.html#aubionotes>
 *  <https://github.com/aubio/aubio/blob/master/examples/aubionotes.c>
 */

use hound::{WavReader, Error};
use aubio::PitchMode;
use aubio::{Pitch, Notes};

fn process(samples: Vec<f32>, spec: &hound::WavSpec) {
    let buff_size = 4096;
    let hop_size = 1024;

    let onset_minioi = 0.0;
    let silence_threshold = 0.0;
    let release_drop = 0.0;

    println!("PROCESSING: {} samples", samples.len());

    let notes = Notes::new(buff_size, hop_size, spec.sample_rate).unwrap()
        //.with_minioi_ms(onset_minioi)
        //.with_silence(silence_threshold)
        //.with_release_drop(release_drop)
        .do_result(samples);

    println!("Finished");
    println!("{:?}", notes);
}

fn read_wav_file(path: &str) -> Result<(), Error> {
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    let samples: Vec<f32> = samples.iter().map(|&s| s as f32 / 32768.0).collect();

    process(samples, &spec);

    Ok(())
}

fn main() {
    read_wav_file("examples/data/c3-major-scale-piano.wav").unwrap();
}
