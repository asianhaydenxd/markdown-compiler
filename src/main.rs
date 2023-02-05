use clap::Parser;

mod file_reader;
mod compiler;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of target file
    filename: String,
}


fn main() {
    let args = Args::parse();
    
    match file_reader::read_file(args.filename) {
        Ok(filetext) => compiler::compile(filetext),
        Err(error) => panic!("{}", error),
    }
}
