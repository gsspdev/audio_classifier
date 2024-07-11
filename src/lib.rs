// src/lib.rs
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioInfo {
    kind: Option<Kind>,
    inst: Option<Inst>,
    key: Option<Key>,
    bpm: Option<BPM>,
    length: Length,
}

impl AudioInfo {
    // Constructor for AudioInfo
    pub fn new(
        kind: Option<Kind>,
        inst: Option<Inst>,
        key: Option<Key>,
        bpm: Option<BPM>,
        length: Length,
    ) -> Self {
        AudioInfo {
            kind,
            inst,
            key,
            bpm,
            length,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Shot,
    Loop,
    Stem,
    Track,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Inst {
    Drum,
    Bass,
    Vox,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BPM(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Length(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Root {
    A,
    Bf,
    B,
    Cf,
    C,
    Df,
    D,
    Ef,
    E,
    F,
    G,
    Af,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    Major,
    Minor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key {
    pub root: Root,
    pub scale: Scale,
}

impl fmt::Display for AudioInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AudioInfo {{ kind: {:?}, inst: {:?}, key: {:?}, bpm: {:?}, length: {:?} }}",
            self.kind.unwrap_or(Kind::Track), // Provide a default value or handle None case
            self.inst.unwrap_or(Inst::Other), // Provide a default value or handle None case
            self.key.unwrap_or(Key {
                root: Root::C,
                scale: Scale::Major
            }), // Provide a default value or handle None case
            self.bpm.unwrap_or(BPM(120)),     // Provide a default value or handle None case
            self.length
        )
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.root, self.scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn audio_info_test() {
        let audio_info_example: AudioInfo;
        audio_info_example = AudioInfo::new(
            Some(Kind::Loop),
            Some(Inst::Bass),
            Some(Key {
                root: Root::D,
                scale: Scale::Minor,
            }),
            Some(BPM(120)),
            Length(14),
        );
        println!("example AudioInfo type: {}", audio_info_example);
    }
}
