use std::{fs::File, io::Read};

/// start the game
pub fn play() -> anyhow::Result<()> {
    let mut buf = String::new();
    let _ = File::open("./quiz.json")?.read_to_string(&mut buf)?;
    Ok(())
}
