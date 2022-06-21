use std::env;
// use csv::Error;
use std::error::Error;
use std::process;
#[cfg(not(feature = "tokio"))]
// use futures::stream::StreamExt;
#[cfg(not(feature = "tokio"))]
use async_std::fs::File;
#[cfg(feature = "tokio")]
use tokio1 as tokio;
#[cfg(feature = "tokio")]
use tokio_stream::StreamExt;
use async_std::stream::StreamExt;
#[cfg(feature = "tokio")]
use tokio::fs::File;
// extern crate tokio;
#[derive(Debug)]
struct Cli {
    filename: String,
    path: std::path::PathBuf,
}

async fn read_csv_file(filename:&str) -> Result<(), Box<dyn Error>>{
    let mut csv_file_reader = csv_async::AsyncReader::from_reader(
            File::open(filename).await?
        );
//     let mut records = rdr.records();
    while let Some(record) = csv_file_reader.records().next().await {
        let record = record?;
         println!("Record: {:?}", record.get(1));
       }
    Ok(())
}

#[cfg(not(feature = "tokio"))]
fn main() {
//     let args: Vec<String> = env::args().collect();
    let filename = env::args().nth(1).expect("no filename given");
    let path = env::args().nth(2).expect("no path given");
    let args = Cli {
        filename: filename,
        path: std::path::PathBuf::from(path),
    };

    async_std::task::block_on(async {
        if let Err(err) = read_csv_file(
            "/Users/christogoosen/Downloads/2022-pnb-daily.csv",
        ).await {
            println!("error running filter_by_region: {}", err);
            process::exit(1);
        }
    });

    println!("Filename {:?}", args.filename);
    println!("Path: {:?}", args.path);
    println!("Hello, world!");
}
