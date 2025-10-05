use arboard::Clipboard;
use clap::Parser;
use lipsum::{lipsum, lipsum_words_with_rng};
use rand::thread_rng;
use std::{fmt::Write, thread::sleep, time::Duration};

/// Generate some lorem-ipsum placeholder text.
#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    /// Number of paragraphs to generate.
    #[arg(long, short, default_value_t = 1)]
    pub paragraph: usize,
    /// Number of words per paragraph.
    #[arg(long, short, default_value_t = 64)]
    pub word: usize,
    /// Whether to add empty lines between paragraphs.
    #[arg(long, short, default_value_t = true)]
    pub newline: bool,
    /// Whether to copy the generated text to clipboard.
    #[arg(long, short, default_value_t = true)]
    pub clipboard: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut text = String::new();
    for i in 0..cli.paragraph {
        let new = if i == 0 {
            lipsum(cli.word)
        } else {
            lipsum_words_with_rng(thread_rng(), cli.word)
        };
        writeln!(text, "{new}").unwrap();
        if cli.newline {
            writeln!(text, "").unwrap();
        }
    }
    if cli.newline {
        print!("{text}");
    } else {
        println!("{text}");
    }
    if cli.clipboard {
        if let Ok(mut clip) = Clipboard::new() {
            if clip.set_text(text).is_ok() {
                sleep(Duration::from_millis(1));
                eprintln!("--- copied to clipboard ---")
            }
        }
    }
}
