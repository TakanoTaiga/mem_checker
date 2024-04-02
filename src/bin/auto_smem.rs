use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;

fn main() {
    let _ = create_dir_all("data");
    loop {
        let output = Command::new("smem")
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute command");   

        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines = output_str.lines().skip(1);

        for line in lines {
            let columns: Vec<&str> = line.split_whitespace().collect();

            let uss = columns.get(5).and_then(|&s| s.parse::<u32>().ok()).unwrap_or(0);
            let pid = columns.get(0).unwrap_or(&"");
            
            let file_name = format!("./data/{}.csv",pid);
                let path = Path::new(&file_name);
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(path)
                    .expect("Failed to open file");

            writeln!(file, "{}",uss)
                .expect("Failed to write to file");
        }
        thread::sleep(Duration::from_secs(1));
    }
}
