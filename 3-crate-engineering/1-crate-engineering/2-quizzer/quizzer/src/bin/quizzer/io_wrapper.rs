use quizzer::question::Question;

/// io wrapper
#[allow(unused)]
pub trait IOWrapper {
    fn read_line(&mut self) -> Option<String>;

    fn write(&mut self, message: String);

    fn write_line(&mut self, message: String);

    fn read_json_file(&mut self) -> Vec<Question>;

    fn write_json_file(&mut self, questions: Vec<Question>);
}
