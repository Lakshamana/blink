use crate::led::Led;

const CYCLES_PER_MS: u32 = 72_000;
const SPEED_FACTOR: u32 = 2;

pub fn delay_ms(ms: u32) {
    cortex_m::asm::delay(CYCLES_PER_MS / SPEED_FACTOR * ms);
}

trait MorseChar {
    fn to_morse(&self) -> &'static str;
}

impl MorseChar for char {
    fn to_morse(&self) -> &'static str {
        match self {
            'A' | 'a' => ".-",
            'B' | 'b' => "-...",
            'C' | 'c' => "-.-.",
            'D' | 'd' => "-..",
            'E' | 'e' => ".",
            'F' | 'f' => "..-.",
            'G' | 'g' => "--.",
            'H' | 'h' => "....",
            'I' | 'i' => "..",
            'J' | 'j' => ".---",
            'K' | 'k' => "-.-",
            'L' | 'l' => ".-..",
            'M' | 'm' => "--",
            'N' | 'n' => "-.",
            'O' | 'o' => "---",
            'P' | 'p' => ".--.",
            'Q' | 'q' => "--.-",
            'R' | 'r' => ".-.",
            'S' | 's' => "...",
            'T' | 't' => "-",
            'U' | 'u' => "..-",
            'V' | 'v' => "...-",
            'W' | 'w' => ".--",
            'X' | 'x' => "-..-",
            'Y' | 'y' => "-.--",
            'Z' | 'z' => "--..",
            _ => "",
        }
    }
}

pub struct MorseTiming {
    pub dot_ms: u32,
}

impl MorseTiming {
    pub const fn new(dot_ms: u32) -> Self {
        Self { dot_ms }
    }

    pub fn symbol_gap_ms(&self) -> u32 {
        self.dot_ms
    }

    pub fn dash_ms(&self) -> u32 {
        3 * self.dot_ms
    }

    pub fn letter_gap_ms(&self) -> u32 {
        3 * self.dot_ms
    }

    pub fn word_gap_ms(&self) -> u32 {
        7 * self.dot_ms
    }
}

pub fn blink_morse<PIN>(
    led: &mut Led<PIN>,
    text: &str,
    timing: &MorseTiming,
) -> Result<(), PIN::Error>
where
    PIN: embedded_hal::digital::v2::OutputPin,
{
    for letter in text.chars() {
        let char_morse = letter.to_morse();
        if !char_morse.is_empty() {
            blink_pattern(led, char_morse, timing)?;
            delay_ms(timing.letter_gap_ms());
        } else if letter == ' ' {
            delay_ms(timing.word_gap_ms());
        }
    }

    Ok(())
}

pub fn blink_pattern<PIN>(
    led: &mut Led<PIN>,
    pattern: &str,
    timing: &MorseTiming,
) -> Result<(), PIN::Error>
where
    PIN: embedded_hal::digital::v2::OutputPin,
{
    for (idx, symbol) in pattern.chars().enumerate() {
        match symbol {
            '.' => {
                led.on()?;
                delay_ms(timing.dot_ms);
                led.off()?;
            }
            '-' => {
                led.on()?;
                delay_ms(timing.dash_ms());
                led.off()?;
            }
            _ => continue,
        }

        if idx < pattern.len() - 1 {
            delay_ms(timing.symbol_gap_ms());
        }
    }

    let _ = led.off();
    Ok(())
}
