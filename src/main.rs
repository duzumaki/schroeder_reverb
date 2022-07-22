use hound;
use std::path::Path;

fn read_wav(path: &Path) -> Vec<i16> {
    let mut reader = hound::WavReader::open(path).unwrap();
    let samples = reader.samples::<i16>().map(|s| s.unwrap()).collect();
    samples
}

fn comb_filter(
    samples: Vec<i16>,
    samples_length: u16,
    delay: f32,
    decay_factor: f32,
    sample_rate: u32,
) -> Vec<f32> {
    // Comb filtering is the mixing of indentical audio signals with a slight delay.
    // In other words it creates an "echo" effect.

    // from the number of samples per millisecond
    let delay_samples = (delay * (sample_rate / 1000) as f32) as u16;

    let mut comb_filter_samples: Vec<f32> = samples.iter().map(|&x| x as f32).collect();

    for i in 0..(samples_length - delay_samples) {
        comb_filter_samples[(i + delay_samples) as usize] +=
            comb_filter_samples[i as usize] as f32 * decay_factor
    }

    comb_filter_samples
}

fn main() {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let path: &'static str = "./StarWars60.wav";
    let wav_path = Path::new("./foo/bar.txt");
    let samples: Vec<i16> = read_wav(&wav_path);
}
