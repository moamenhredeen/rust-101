use std::{fs::File, io};

use add::add;
use anyhow::Ok;
use clap::{Parser, Subcommand};

mod add;
mod play;

#[derive(Parser)]
#[command(
    name = "quizzer",
    version = "1.0",
    about = "quiz cli",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "start playing")]
    Play,

    #[command(about = "create questions")]
    Add {
        #[arg(short, long)]
        file: String,
    },
}

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout().lock();
    let mut stdin = io::stdin().lock();

    let cli = Cli::parse();
    _ = match &cli.command {
        Some(Commands::Add { file }) => {
            let file = File::create(file)?;
            println!("file: {:?}", file);
            _ = add(&mut stdin, &mut stdout);
            Ok(())
        } //start_editor()
        Some(Commands::Play) => {
            _ = play::play();
            Ok(())
        }
        _ => Ok(()),
    };

    Ok(())
}
