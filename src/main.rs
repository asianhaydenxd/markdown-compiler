use clap::Parser;

mod file_reader;
mod compiler;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Names of target files
    filenames: Vec<String>,
}


fn main() {
    let args = Args::parse();
    
    for filename in args.filenames {
        match file_reader::read_file(filename) {
            Ok(filetext) => compiler::compile(filetext),
            Err(error) => panic!("{}", error),
        }
    }
}
