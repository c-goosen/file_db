use std::env;
// extern crate tokio;
#[derive(Debug)]
struct Cli {
    filename: String,
    path: std::path::PathBuf,
}



fn main() {
//     let args: Vec<String> = env::args().collect();
    let filename = env::args().nth(1).expect("no filename given");
    let path = env::args().nth(2).expect("no path given");
    let args = Cli {
        filename: filename,
        path: std::path::PathBuf::from(path),
    };

    println!("Filename {:?}", args.filename);
    println!("Path: {:?}", args.path);
    println!("Hello, world!");
}
