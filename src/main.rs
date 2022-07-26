use hound;
use std::path::Path;

fn read_wav(path: &Path) -> Vec<i16> {
    let mut reader = hound::WavReader::open(path).unwrap();
    let samples = reader.samples::<i16>().map(|s| s.unwrap()).collect();
    samples
}

fn comb_filter(
    samples: &Vec<i16>,
    samples_length: u16,
    delay: f32,
    decay_factor: f32,
    sample_rate: u32,
) -> Vec<f32> {
    // comb filtering is the mixing of indentical audio signals with a slight delay.
    // In other words it creates an "echo" effect
    // a comb filter exponentially decays the impulse response

    // number of samples of delay
    let delay_samples = (delay * (sample_rate / 1000) as f32) as u16;

    let mut comb_filter_samples: Vec<f32> = samples.iter().map(|&x| x as f32).collect();

    for i in 0..(samples_length - delay_samples) {
        comb_filter_samples[(i + delay_samples) as usize] +=
            comb_filter_samples[i as usize] as f32 * decay_factor
    }

    comb_filter_samples
}

fn all_pass_filter(samples: Vec<f32>, samples_length: u16, sample_rate: u32) -> Vec<f32> {
    let delay_samples = (89.27 * (sample_rate / 1000) as f32) as i16;
    let mut all_pass_filter_samples: Vec<f32> = vec![0.0; samples_length as usize];
    let decay_factor = 0.131;

    for i in 0..samples_length {
        all_pass_filter_samples[i as usize] = samples[i as usize];

        if i - delay_samples >= 0 {
            all_pass_filter_samples[i as usize] +=
                -decay_factor * all_pass_filter_samples[(i - delay_samples) as usize];
        }

        if i - delay_samples >= 1 {
            all_pass_filter_samples[i as usize] +=
                decay_factor * all_pass_filter_samples[(i - (1 + delay_samples)) as usize];
        }
    }

    // Smoothing and normalisation of samples to prevent clipping.
    let mut value = all_pass_filter_samples[0];
    let mut max = 0.0;

    for i in 0..samples_length {
        if all_pass_filter_samples[i as usize].abs() > max {
            max = all_pass_filter_samples[i as usize].abs();
        }
    }

    for i in 0..all_pass_filter_samples.len() {
        let current = all_pass_filter_samples[i as usize];
        value = (value + (current - value)) / max;

        all_pass_filter_samples[i as usize] = value;
    }

    all_pass_filter_samples
}

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 22000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mix_percent = 50.0;

    let path: &'static str = "./StarWars60.wav";
    let wav_path = Path::new(path);
    let samples: Vec<i16> = read_wav(&wav_path);

    // use comb filter
    // let frame_size = spec.sample_rate * (spec.channels as u32);
    let buffer_size = samples.len();

    let comb_filter_samples1 =
        comb_filter(&samples, buffer_size as u16, 78.9, 0.45, spec.sample_rate);
    let comb_filter_samples2 = comb_filter(
        &samples,
        buffer_size as u16,
        78.9 - 11.73,
        0.45 - 0.1313,
        spec.sample_rate,
    );
    let comb_filter_samples3 = comb_filter(
        &samples,
        buffer_size as u16,
        78.9 - 19.31,
        0.45 - 0.2743,
        spec.sample_rate,
    );
    let comb_filter_samples4 = comb_filter(
        &samples,
        buffer_size as u16,
        78.9 - 7.97,
        0.45 - 0.31,
        spec.sample_rate,
    );

    let mut comb_filter_output: Vec<f32> = vec![0.0; buffer_size];

    // combine comb filters
    for i in 0..buffer_size {
        comb_filter_output[i] = comb_filter_samples1[i]
            + comb_filter_samples2[i]
            + comb_filter_samples3[i]
            + comb_filter_samples4[i]
    }

    // add dry/wet mix
    let mut mixed_audio: Vec<i16> = vec![0; buffer_size];
    for i in 0..buffer_size {
        mixed_audio[i] = (((100.0 - mix_percent) * samples[i] as f32)
            + (mix_percent * comb_filter_output[i])) as i16
    }

    let mixed_audio_f32 = mixed_audio.iter().map(|&x| x as f32).collect();

    // apply all pass filters
    let all_pass_filter_samples1 =
        all_pass_filter(mixed_audio_f32, buffer_size as u16, spec.sample_rate);
    let all_pass_filter_samples2 = all_pass_filter(
        all_pass_filter_samples1,
        buffer_size as u16,
        spec.sample_rate,
    );

    //  write to wav file
    let mut writer = hound::WavWriter::create(format!("test.wav"), spec).unwrap();

    for sample in all_pass_filter_samples2 {
        writer.write_sample(sample).unwrap();
    }
    writer.finalize().unwrap();
}
