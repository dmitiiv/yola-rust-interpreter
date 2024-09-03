use std::process;

pub enum ErrorTypes {
    SynErr,
    TypeErr,
}

impl ErrorTypes {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorTypes::SynErr => "SynErr",
            ErrorTypes::TypeErr => "TypeErr",
        }
    }
}

pub struct Report {}

impl Report {
    pub fn error(code: Option<i32>, err: &str, line: usize, message: String) {
        let code = code.unwrap_or(1);

        println!("[ Line {}] Error {}: {}", line, err, message);

        process::exit(code);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn report_error() {
        let line = 1;
        let mess = String::from("message");
        let error = String::from("error");

        let code = Some(2);

        assert_eq!(
            (),
            Report::error(code, ErrorTypes::SynErr.as_str(), line, mess)
        );
    }
}
