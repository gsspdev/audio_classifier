// src/main.rs
use hound::WavReader;
use std::fs;
use std::io;
// use std::path::Path;

use audio_classifier::{AudioInfo, Inst, Key, Kind, Length, Root, Scale, BPM};

fn collect_wav_files(dir: &str) -> io::Result<Vec<String>> {
    let mut wav_files = Vec::new();
    let entries = fs::read_dir(dir)?;

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

fn get_wav_length(file_path: &str) -> io::Result<u32> {
    let reader = WavReader::open(file_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let spec = reader.spec();
    let duration = reader.duration();
    let length_in_seconds = duration / spec.sample_rate;
    Ok(length_in_seconds)
}

fn main() -> io::Result<()> {
    // use audio_classifier::AudioInfo;
    
    let audio_info_test = AudioInfo::new(
        Some(Kind::Loop),
        Some(Inst::Bass),
        Some(Key {
            root: Root::D,
            scale: Scale::Minor,
        }),
        Some(BPM(120)),
        Length(14),
    );
    
    println!("example AudioInfo type: {:?}", audio_info_test);
    let wav_files = collect_wav_files("assets/audio/")?;
    for file in wav_files {
        match get_wav_length(&file) {
            Ok(length) => println!("{}: {} seconds", file, length),
            Err(e) => eprintln!("Error reading {}: {}", file, e),
        }
    }

    

    Ok(())
}
