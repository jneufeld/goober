use clap::Parser;
use emojis::Emoji;

#[derive(Parser, Debug)]
#[clap(name = "ke", about = "Demonstration of Kaley Encoding", long_about = None)]
struct Args {
    /// Prints the name (i.e. characters) of each emoji used in the solution as a reference
    #[clap(long, action)]
    reference: bool,

    /// The word to be encoded (must be a single, non-whitespace-separated word)
    #[clap(short, long, value_parser)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let input = args.input.to_ascii_lowercase();
    let reference = args.reference;

    display(&input, reference);
}

/// Prints the result and optional reference to the terminal
fn display(input: &str, reference: bool) {
    let lexicon = build_lexicon();
    let result = encode(input, &lexicon);

    if reference {
        println!("Reference:");

        for (rune, _) in &result {
            println!("{} {}", rune.emoji, rune.letters);
        }

        println!();
    }

    println!("Result:");

    for (rune, count_indicator) in &result {
        match count_indicator {
            Some(indicator) => print!("{}{}", rune.emoji, indicator),
            None => print!("{}", rune.emoji),
        }
    }

    println!();
}

/// A rune represents an emoji and the characters that describe or name it
struct Rune<'a> {
    emoji: &'a Emoji,
    letters: String,
}

/// The lexicon or corpus or whatever is the set of all symbols in Kaley Encoding. For simplicity, this
/// solution filters all emojis so only those with single-word names are included. The result is a list of
/// runes sorted by their characters. The sorted property is used by the encoding algorithm.
fn build_lexicon<'a>() -> Vec<Rune<'a>> {
    let mut runes = Vec::new();

    for emoji in emojis::iter() {
        let name = emoji.name().to_ascii_lowercase();

        if name.contains(' ') {
            continue;
        }

        runes.push(Rune {
            emoji,
            letters: name.to_string(),
        })
    }

    runes.sort_by(|a, b| a.letters.cmp(&b.letters));

    runes
}

/// Performs Kaley encoding on an input string. The result is a list of rune-integer pairs where the integer
/// indicates how many times it is repeated.
fn encode<'a>(
    input: &str,
    lexicon: &'a Vec<Rune<'a>>,
) -> Vec<(&'a Rune<'a>, Option<&'static Emoji>)> {
    let mut result = Vec::new();
    let mut input_index = 0;

    loop {
        if input_index >= input.len() {
            break;
        }

        let mut longest_run = 0;
        let mut best_rune = None;

        for rune in lexicon {
            if input == rune.letters {
                continue;
            }

            let mut rune_index = 0;

            while input.chars().nth(input_index + rune_index)
                == rune.letters.chars().nth(rune_index)
            {
                rune_index += 1;
            }

            if rune_index > longest_run {
                longest_run = rune_index;
                best_rune = Some(rune);
            }
        }

        result.push((
            best_rune.expect("No sequence available"),
            run_indicator(longest_run),
        ));

        input_index += longest_run;
    }

    result
}

/// Indicate the how many letters of the emoji's name are counted with the color of a heart. If no heart is
/// returned, the `None` variant, then only count one.
///
/// For humans, the color of the heart is the guide: ROY G BIV. Red means two, orange means three, etc.
///
/// These don't need to be removed from the lexicon because their names are all two words (e.g. "red heart").
fn run_indicator(length: usize) -> Option<&'static Emoji> {
    match length {
        1 => None,
        2 => Some(emojis::get("❤").unwrap()),
        3 => Some(emojis::get("🧡").unwrap()),
        4 => Some(emojis::get("💛").unwrap()),
        5 => Some(emojis::get("💚").unwrap()),
        6 => Some(emojis::get("💙").unwrap()),
        7 => Some(emojis::get("💜").unwrap()),
        _ => Some(emojis::get("🖤").unwrap()),
    }
}
