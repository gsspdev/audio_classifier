// src/main.rs
use hound::WavReader;
use std::fs;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;

use audio_classifier::*;

pub fn buf_read {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    let mut stack = Vec::new();
    let mut occurences = HashMap::new();
    while let _ = reader.read_line(&mut line).unwrap() {
        if line.starts_with('#') {
            continue;
        }

        if line.trim_end().is_empty() {
            // end of stack, so emit stack entry
            if !stack.is_empty() {
                occurences.entry(stack.join(";")).or_insert(0) += 1; // counter trackshow many times we've seen a particular stack
            }
            stack.clear();
            continue;
        }

       // one of two types of lines are possible
       // either, we have an event line
       // or, we have a stack line
    }
}

// operate_on(BufReader::new(stdin)).unwrap();

// fn operate_on(r: BufReader) -> io::Result<()> {
//     let mut line = String::new();
//     while let _ = r.read_line(&mut line)? {
//         if line.starts_with('#')
//     }
// }



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
    let wav_files = collect_wav_files("assets/audio/")?;
    for file in wav_files {
        match get_wav_length(&file) {
            Ok(length) => println!("{}: {} seconds", file, length),
            Err(e) => eprintln!("Error reading {}: {}", file, e),
        }
    }
    Ok(())
}
