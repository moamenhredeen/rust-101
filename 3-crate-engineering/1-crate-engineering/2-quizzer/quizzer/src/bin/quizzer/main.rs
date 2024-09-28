use std::{
    fs::File,
    io::{self, BufRead, Write},
};

use anyhow::Ok;
use quizzer::{Answer, Question, Quiz};

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "quizzer", version = "1.0", about = "quiz cli")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "start playing")]
    Player,

    #[command(about = "create questions")]
    Editor,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Editor) => {
            let mut quiz = Quiz::new();

            let mut stdin = io::stdin().lock();
            let mut stdout = io::stdout().lock();
            let mut buf = String::new();

            loop {
                // read question
                write!(&mut stdout, "Enter a new question: ")?;
                stdout.flush()?;
                _ = stdin.read_line(&mut buf)?;
                if buf.trim().is_empty() {
                    break;
                }

                trim_newline(&mut buf);
                let mut q = Question::new(buf.trim().to_string());

                loop {
                    // read answer
                    buf.clear();
                    write!(&mut stdout, "Enter Answer: ")?;
                    stdout.flush()?;
                    _ = stdin.read_line(&mut buf)?;
                    // if the user did not enter anything exit
                    if buf.trim().is_empty() {
                        break;
                    }

                    trim_newline(&mut buf);
                    let mut ans = Answer::from_string(buf.trim().to_string());

                    // read if the answer a correct answer
                    buf.clear();
                    write!(&mut stdout, "was this a correct answer [yes/NO]: ")?;
                    stdout.flush()?;
                    _ = stdin.read_line(&mut buf)?;

                    trim_newline(&mut buf);
                    if buf.trim().to_lowercase().eq("yes") {
                        ans.correct(true);
                    }

                    q.add_answer(ans);
                }

                // save question
                quiz.add_question(q)?;

                let quiz_json = serde_json::to_string(&quiz)?;
                let mut output_file = File::create("./quiz.json")?;
                write!(&mut output_file, "{}", quiz_json)?;
            }

            println!("{:?}", quiz);
            Ok(())
        }
        Some(Commands::Player) => {
            println!("player");
            Ok(())
        }
        None => {
            unreachable!()
        }
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
