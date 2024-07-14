use hound::{WavReader, WavSpec};
use std::fs;
use std::io;

fn collect_wav_files(dir: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(dir)?;
    let mut wav_files = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "wav" {
                    if let Some(path_str) = path.to_str() {
                        wav_files.push(path_str.to_string());
                    }
                }
            }
        }
    }
    Ok(wav_files)
}

fn get_wav_length(file_path: &str) -> io::Result<u32>{
    let reader = WavReader::open(file_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let spec = reader.spec();
    let duration = reader.duration(); // potentially redundant function call
    let length_in_seconds = duration / spec.sample_rate;
    Ok(length_in_seconds)
}

fn get_wav_specs(file_path: &str) -> io::Result<WavSpec> {
    let reader = WavReader::open(file_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let spec = reader.spec();
    let _duration = reader.duration();
    Ok(spec)
}

fn main() -> io::Result<()> {
    // use audio_classifier::AudioInfo;
    let wav_files = collect_wav_files("assets/audio/")?;
    for file in &wav_files {
        match get_wav_length(&file) {
            Ok(length) => println!("{}: {} seconds", file, length),
            Err(e) => eprintln!("Error reading {}: {}", file, e),
        }
        match get_wav_specs(&file) {
            Ok(specs) => println!("{}: {:?}", file, specs),
            Err(e) => eprintln!("Error reading {}: {}", file, e),
        }
    }
    Ok(())
}
