use clap::Parser;
use colored::Colorize;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String,
    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,
    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    cat_file: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let message = options.message;

    let eye = if options.dead { "x" } else { "o" };

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
    }

    match &options.cat_file {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path).with_context(
                || format!("Could not read file {:?}", path)
            )?;
            println!("{}", message.bright_yellow().underline().on_purple());
            print_the_cat_from_template(&cat_template, eye);
        },
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            print_the_cat(eye);
        }
    }
    Ok(())
}

fn print_the_cat_from_template(cat_template: &str, eye: &str) {
    let eye = format!("{}", eye.red().bold());
    let cat_picture = cat_template.replace("{eye}", &eye);
    println!("{}", &cat_picture);
}

fn print_the_cat(eye: &str) {
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {eye} {eye} )", eye=eye.red().bold());
    println!("    =( I )=");
}