use std::process;

struct Report {}

impl Report {
    pub fn error(line: i32, err: String, message: String, code: Option<i32>) {
        let code = code.unwrap_or(1);

        println!("[ Line {}] Error {}: {}", line, err, message);

        process::exit(code);
    }
}
