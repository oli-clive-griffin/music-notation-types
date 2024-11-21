//// this is a silly little challenge:
//// Can I make a nice set of types and data structures for representing music notation?
//// What are the tradeoffs
use std::collections::HashSet;

// should this be called "measure" or something?
enum NoteLetter {
    /// whatever this is called, forgot, no wifi
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

type NoteOctave = u8;

enum SemitoneModifier {
    Flat,
    Sharp,
}

struct Note(NoteLetter, Option<SemitoneModifier>, NoteOctave);

/// duration with corresponding duration as u8
enum BaseNoteDuration {
    /// whatever this is called, forgot, no wifi
    FullNote = 0b1000_0000,
    Minum = 0b0100_0000,
    Crotchet = 0b0010_0000,
    Quaver = 0b0001_0000,
    SemiQuaver = 0b0000_1000,
    DemiSemiQuaver = 0b0000_0100,
    HemiDemiSemiQuaver = 0b0000_0010,
}

struct NoteDuration {
    base_duration: BaseNoteDuration,
    dotted: bool,
}

struct Accent(
    String, // can't remember the accent types at the moment
);

struct HeldNote {
    note: Note,
    duration: NoteDuration,
    accents: HashSet<Accent>,
}

/// binary numbers are a very good analagy for the way we think of note
/// durations which is basically base 2, In this case the most significant
/// bit represents a full note
type Duration = u8;

struct NoteInPlace {
    note: HeldNote,
    offset: Duration,
}

type NotesPerBar = usize;

struct TimeSignature(NotesPerBar, BaseNoteDuration);

struct VoiceBar {
    placed_notes: Vec<NoteInPlace>,
}

struct Bar<'a> {
    time_signature: &'a TimeSignature,
    key_signature: &'a KeySignature,
    voices: Vec<VoiceBar>,
}

struct KeySignature {
    modifiers: HashSet<SemitoneModifier>,
}

struct Piece<'a> {
    bars: Vec<Bar<'a>>,
}

macro_rules! four_four {
    () => {
        TimeSignature(4, BaseNoteDuration::Crotchet)
    };
}
    

fn example() {
    let c_major = KeySignature {
        modifiers: HashSet::<SemitoneModifier>::new(),
    };

    let four_four = four_four!();

    let piece = Piece {
        bars: vec![Bar {
            time_signature: &four_four,
            key_signature: &c_major,
            voices: vec![VoiceBar {
                placed_notes: vec![
                    NoteInPlace {
                        note: HeldNote {
                            note: Note(NoteLetter::A, Some(SemitoneModifier::Sharp), 3),
                            accents: HashSet::new(),
                            duration: NoteDuration {
                                base_duration: BaseNoteDuration::Quaver,
                                dotted: true,
                            },
                        },
                        offset: 0b0011_0000,
                    },
                    NoteInPlace {
                        note: HeldNote {
                            note: Note(NoteLetter::A, Some(SemitoneModifier::Sharp), 3),
                            accents: HashSet::new(),
                            duration: NoteDuration {
                                base_duration: BaseNoteDuration::Quaver,
                                dotted: true,
                            },
                        },
                        offset: 0b0011_0000,
                    },
                ],
            }],
        }],
    };
}
