use std::io::{BufRead, Write};

use quizzer::question::Question;

/// start the game
pub fn play<F, R, W>(file: &mut F, reader: &mut R, writer: &mut W) -> anyhow::Result<()>
where
    F: BufRead,
    R: BufRead,
    W: Write,
{
    let mut lines = reader.lines();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let question_list: Vec<Question> = serde_json::from_str(&file_content)?;
    let mut index = 0;
    for question in question_list {
        writeln!(writer, "{}", question.content)?;
        for ans in question.incorrect_answers {
            index += 1;
            writeln!(writer, "{}. {}", index, ans)?;
        }
        writeln!(writer, "{}. {}", index, question.correct_answer)?;

        writeln!(writer, "Choose the right answer ?")?;
        let res = match lines.next() {
            Some(Ok(i)) => match i.trim().parse::<u8>() {
                Ok(1) => "wrong",
                Ok(2) => "wrong",
                Ok(3) => "wrong",
                Ok(4) => "wrong",
                Ok(_) => "wrong",
                Err(_) => "please enter a valid number",
            },
            Some(Err(_)) => "error while reading the answer",
            None => "please enter your answer",
        };

        writeln!(writer, "{}", res)?;
    }
    Ok(())
}
