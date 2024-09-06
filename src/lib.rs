use std::{fs, process::Command};

pub struct Config {
    pub videopath: String,
    pub outpath: String,
    pub maxsize: usize, // in MB
}

#[derive(Debug)]
pub struct MetaData {
    pub bitrate: f64, // in MB
    pub length: f64,  // in seconds
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let videopath = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get video path"),
        };
        let outpath = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get output file path"),
        };
        let maxsize = match args.next() {
            Some(args) => args.trim().parse().unwrap(),
            None => return Err("Didn't get size(Mb) limit to trim video"),
        };
        Ok(Config {
            videopath,
            outpath,
            maxsize,
        })
    }
}

impl MetaData {
    pub fn build(config: &Config) -> Result<MetaData, Box<dyn std::error::Error>> {
        let command_output = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-show_entries")
            .arg("format=duration")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(&config.videopath)
            .output()?;
        let length = String::from_utf8_lossy(&command_output.stdout);
        let length: f64 = length.trim().parse().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to parse length")
        })?;
        let command_output = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-show_entries")
            .arg("format=bit_rate")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(&config.videopath)
            .output()?;
        let bitrate = String::from_utf8_lossy(&command_output.stdout);
        let bitrate: f64 = bitrate.trim().parse().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to parse bitrate")
        })?;
        if let Ok(_) = fs::create_dir(&config.outpath) {
            println!("[!] Output directory {} created", config.outpath)
        };
        Ok(MetaData {
            bitrate: ((bitrate / 1000.0) / 8.0) / 1000.0,
            length,
        })
    }

    pub fn calculate(&self, size: &f64) -> (f64, usize) {
        let trim_duration: f64 = size / self.bitrate;
        let total_parts = (self.length / trim_duration) as usize;
        (trim_duration, total_parts)
    }
}

pub fn trim(config: &Config, trim_duration: f64) -> Result<bool, Box<dyn std::error::Error>> {
    let mut fout = config.outpath.to_string();
    fout.push_str("/%d.mp4");
    let mut command = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(&config.videopath)
        .arg("-c")
        .arg("copy")
        .arg("-map")
        .arg("0")
        .arg("-f")
        .arg("segment")
        .arg("-segment_time")
        .arg(format!("{}", trim_duration))
        .arg("-reset_timestamps")
        .arg("1")
        .arg(&fout)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?;
    let status = command.wait()?;
    Ok(status.success())
}
