use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("args len {:?}", &args)
    } else if args.len() == 1 {
        println!("args len {:?}", &args);
        run_file(&args[0]);
    } else {
        println!("args len {:?}", &args)
    }
}

fn run_file(path: &str) {
    let mut reader = SourceReader::new();

    reader
        .read_sources(path)
        .expect("Error during read sources");

    let _source = reader.get_sources();
}

struct SourceReader {
    buffer: Vec<u8>,
}

impl SourceReader {
    fn new() -> SourceReader {
        SourceReader { buffer: vec![] }
    }

    fn read_sources(&mut self, path: &str) -> Result<(), String> {
        let mut file = File::open(path).unwrap();
        self.buffer.clear();

        file.read_to_end(&mut self.buffer);

        Ok(())
    }

    fn get_sources(&self) -> &Vec<u8> {
        &self.buffer
    }
}
