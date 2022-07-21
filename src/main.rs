use hound;
use std::path::Path;


fn read_wav(path: &Path) -> Vec<i16> {
	let mut reader = hound::WavReader::open(path).unwrap();
	let samples = reader.samples::<i16>().map(|s| s.unwrap()).collect();
	samples
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
