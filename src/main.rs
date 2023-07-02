use clap::Parser;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use futures::executor::block_on;
use mkdirp::mkdirp;
use path_absolutize::Absolutize;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[arg(short, long)]
    treat_input_as_image_paths: bool,
    #[arg(short, long)]
    input_file: Option<String>,
    #[arg(short, long)]
    left_option: String,
    #[arg(short, long)]
    right_option: String,
    #[arg(short = 'o', long)]
    left_file: Option<String>,
    #[arg(short = 'p', long)]
    right_file: Option<String>,
    #[arg(short, long)]
    gui: bool,
}

fn write_to_file(filename: &String, line: &String) {
    let filepath = Path::new(filename).absolutize().unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&filepath)
        .unwrap();

    if let Err(e) = mkdirp(filepath.parent().unwrap()) {
        eprintln!("Couldn't write to file {}\nError: {:?}", filename, e);
        process::exit(1);
    }

    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file {}\nError: {:?}", filename, e);
        process::exit(1);
    }
}

fn classify(candidate: &String, selection: &String) {
    let cli = Cli::parse();

    if selection == &cli.left_option {
        match cli.left_file {
            Some(file) => {
                write_to_file(&file, candidate);
            }
            None => {
                println!("{} - {}", selection, candidate)
            }
        }
    } else if selection == &cli.right_option {
        match cli.right_file {
            Some(file) => {
                write_to_file(&file, candidate);
            }
            None => {
                println!("{} - {}", selection, candidate)
            }
        }
    } else {
        println!("Item {} was somehow classified as an option that was not provided\nClassification: {}\nOptions: {} or {}", candidate, selection, cli.left_option, cli.right_option)
    }
}

fn run_classification_tui(candidates: Vec<String>, options: Vec<&String>) -> std::io::Result<()> {
    for candidate in candidates {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(&candidate)
            .items(&options)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        match selection {
            Some(index) => classify(&candidate, &options[index]),
            None => {
                println!("No input");
                process::exit(0);
            }
        }
    }

    Ok(())
}

fn run_classification_gui(_candidates: Vec<String>, _options: Vec<&String>) -> std::io::Result<()> {
    println!("GUI not implemented yet, check back soon!");
    Ok(())
}

async fn run() -> std::io::Result<()> {
    let cli = Cli::parse();

    let input: Box<dyn BufRead> = match &cli.input_file {
        Some(input_file) => match File::open(input_file) {
            Ok(file) => Box::new(BufReader::new(file)),
            Err(e) => {
                println!("Could not open file: {}\nError: {:?}", input_file, e);
                process::exit(1);
            }
        },
        None => Box::new(BufReader::new(std::io::stdin())),
    };

    let candidates: Vec<String> = input.lines().map(|v| v.unwrap()).collect();

    let options = vec![&cli.left_option, &cli.right_option];

    match cli.gui {
        true => run_classification_gui(candidates, options),
        false => run_classification_tui(candidates, options),
    }
}

fn main() {
    let future = run();
    match block_on(future) {
        Ok(_) => {}
        Err(e) => {
            println!("Something went wrong\n{:?}", e);
            process::exit(1);
        }
    }
}
