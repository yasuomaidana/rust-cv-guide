use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match cvg_tools::run(&args, Path::new(".")) {
        Ok(message) => println!("{message}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
