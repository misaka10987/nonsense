use arboard::Clipboard;
use clap::Parser;
use lipsum::{lipsum, lipsum_title_with_rng, lipsum_words_with_rng};
use rand::thread_rng;
use std::{borrow::Cow, fmt::Write, thread::sleep, time::Duration};

/// Lorem ipsum placeholder text generator.
#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    /// Number of paragraphs to generate.
    #[arg(long, short, default_value_t = 1)]
    pub paragraph: usize,
    /// Number of words per paragraph.
    #[arg(long, short, default_value_t = 64)]
    pub word: usize,
    /// Disable the empty lines between paragraphs.
    #[arg(long, default_value_t = false)]
    pub no_newline: bool,
    /// Disable clipboard addition integration.
    #[arg(long, default_value_t = false)]
    pub no_clipboard: bool,
    /// Title generation mode.
    #[arg(long, short, default_value_t = false)]
    pub title: bool,
}

fn dummy_text(paragraph: usize, word: usize, newline: bool) -> String {
    let mut text = String::new();
    for i in 0..paragraph {
        let new = if i == 0 {
            lipsum(word)
        } else {
            lipsum_words_with_rng(thread_rng(), word)
        };
        writeln!(text, "{new}").unwrap();
        if newline && i != paragraph - 1 {
            writeln!(text, "").unwrap();
        }
    }
    text
}

fn dummy_title() -> String {
    lipsum_title_with_rng(thread_rng())
}

fn add_to_clipboard<'a>(text: impl Into<Cow<'a, str>>) -> anyhow::Result<()> {
    let mut clip = Clipboard::new()?;
    clip.set_text(text)?;
    sleep(Duration::from_millis(1));
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let text = if cli.title {
        dummy_title()
    } else {
        dummy_text(cli.paragraph, cli.word, !cli.no_newline)
    };
    println!("{text}");
    if !cli.no_clipboard {
        match add_to_clipboard(text) {
            Ok(_) => eprintln!("--- copied to clipboard ---"),
            Err(e) => eprintln!("--- {e}"),
        }
    }
}
