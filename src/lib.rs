// src/lib.rs
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_info_new() {
        let kind = Some(Kind::Loop);
        let inst = Some(Inst::Bass);
        let key = Some(Key { root: Root::D, scale: Scale::Minor });
        let bpm = Some(BPM(120));
        let length = Length(14);

        let audio_info1 = AudioInfo::new
        (
            kind,
            inst,
            key,
            bpm,
            length
        );

        assert_eq!(audio_info1, AudioInfo {
            kind: Some(Kind::Loop),
            inst: Some(Inst::Bass),
            key: Some(Key { root: Root::D, scale: Scale::Minor }),
            bpm: Some(BPM(120)),
            length: Length(14) }
            );
        
    }
}
