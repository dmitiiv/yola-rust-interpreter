use std::process;

struct Report {}

impl Report {
    pub fn new() -> Report {
        Report {}
    }

    pub fn error(&self, code: Option<i32>, err: String, line: i32, message: String) {
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
        let report = Report::new();
        let line = 1;
        let mess = String::from("message");
        let error = String::from("error");

        let code = Some(2);

        assert_eq!((), report.error(code, error, line, mess));
    }
}
