use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::Split;

#[derive(Clone)]
pub struct Input(String);

/// Abstraction around the puzzle input, can provide the input as an iterator over lines or as a str
impl Input {
    pub fn from_lines<I, S>(lines: I) -> Input
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Input(
            lines
                .into_iter()
                .fold(String::new(), |complete, line| {
                    complete + line.as_ref() + "\n"
                })
                .trim_end()
                .to_string(),
        )
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(input: impl Into<String>) -> Input {
        Input(input.into())
    }

    /// Return self without any extra empty newline at the end
    pub fn trim_trailing_newlines(&self) -> Input {
        Input(self.0.trim_end_matches('\n').to_string())
    }

    pub fn load(path: impl AsRef<Path>) -> std::io::Result<Input> {
        let mut input = String::new();
        File::open(path)?.read_to_string(&mut input)?;
        Ok(Input(input))
    }

    /// Get the input as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the input as an iterator of lines
    pub fn as_lines(&self) -> Split<char> {
        self.0.split('\n')
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn test_as_str() {
        // given some input
        let input = Input("a string".to_string());

        // expect input as str to equal "a string"
        assert_eq!("a string", input.as_str())
    }

    #[test]
    fn test_as_lines() {
        // given some input
        let input = Input("a line\nanother line".to_string());

        // expect input as str to equal "a string"
        itertools::assert_equal(input.as_lines(), vec!["a line", "another line"]);
    }

    #[test]
    fn test_from_lines() {
        // given some lines
        let input = ["a line", "another line"];

        // when Input is created from the lines
        let input = Input::from_lines(input);

        // then as_str returns the lines concatenated
        assert_eq!("a line\nanother line", input.as_str());
    }

    #[test]
    fn test_trim_trailing_newlines() {
        // given some input with trailing newlines
        let input = Input("a line\nanother line\n\n".to_string());

        // expect input as lines with trailing newlines removed to only include the non-empty strings
        itertools::assert_equal(
            input.trim_trailing_newlines().as_lines(),
            vec!["a line", "another line"],
        );
    }
}
