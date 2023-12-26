/// functions to convert between musical notes and frequencies
#[derive(Clone, Debug, PartialEq)]
pub struct ParseNoteError;

/// convert note string to midi number
/// note format is `<key>[#]<octave>'
pub fn note2midi(note: &str) -> Result<u8, ParseNoteError> {

    // Parse note into name {A,B,C ...}; octave, and sharp-ness
    let mut chars = note.chars();
    let name = chars.next().ok_or(ParseNoteError{})?;
    let mut octave = chars.next().ok_or(ParseNoteError{})?;
    let mut sharp = false;
    if octave == '#' {
        sharp = true;
        octave = chars.next().ok_or(ParseNoteError{})?;
    }
    let mut buffer = [0; 4];
    let octave = octave.encode_utf8(&mut buffer);

    // Convert into numeric
    let name_number = match name {
        'C' => Ok(0),
        'D' => Ok(2),
        'E' => Ok(4),
        'F' => Ok(5),
        'G' => Ok(7),
        'A' => Ok(9),
        'B' => Ok(11),
        _ => Err(ParseNoteError{})
    }?;

    let octave_number = u8::from_str_radix(&octave,10).or(Err(ParseNoteError{}))?;
    let sharp_number = if sharp {1} else {0};

    // Calculate midi number
    // middle C is 60, A0 is 21
    Ok(name_number + octave_number*12 + sharp_number + 12)
}

// Converts midi note to frequency in Hz
fn midi2freq(midi: u8) -> f32 {
    440.0 * 2.0_f32.powf( (midi as f32 - 69.0) / 12.0 )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn note2midi_test() {
        assert_eq!(note2midi("A0"), Ok(21));
        assert_eq!(note2midi("C4"), Ok(60));
        assert_eq!(note2midi("C#4"), Ok(61));
        assert_eq!(note2midi("???"), Err(ParseNoteError{}));
    }

    #[test]
    fn midi2freq_test() {
        assert_eq!(midi2freq(69), 440.0); // A4
        assert_eq!(midi2freq(81), 880.0); // A5
        assert!((midi2freq(60) - 261.63).abs() < 0.01); // C4
    }
}
