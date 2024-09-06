use std::env;
use std::process;
use videosegmenter::{Config, MetaData};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("[X] Problem parsing arguments: {err}");
        process::exit(1);
    });

    let metadata = MetaData::build(&config).unwrap_or_else(|err| {
        eprintln!("[X] Error while building MetaData struct: {err}");
        process::exit(2);
    });
    // println!("{:?}", metadata);

    let (trim_duration, total_parts) = metadata.calculate(&(config.maxsize as f64));

    println!(
        "[+] {} will be trimmed into {} parts of {:.2}s",
        config.videopath, total_parts, trim_duration
    );
    match videosegmenter::trim(&config, trim_duration) {
        Ok(_) => println!(
            "[=] {} got trimmed at {} dir",
            config.videopath, config.outpath
        ),
        Err(_) => eprintln!(
            "[X] Something went wrong while trimming {}",
            config.videopath
        ),
    }
}
