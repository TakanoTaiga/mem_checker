use std::process::{Command, Stdio};
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::path::Path;


fn main() {
    loop {
        let output = Command::new("ps")
            .args(["aux"])
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8_lossy(&output.stdout);

        for line in output_str.lines().skip(1) {
            let columns: Vec<&str> = line.split_whitespace().collect();
            let user = columns.get(0).unwrap_or(&"");
            let pid = columns.get(1).unwrap_or(&"");

            if *user == "taiga" {
                let rss_str = columns.get(5).unwrap_or(&"0");
                let rss_value: i32 = rss_str.parse().unwrap_or(0);

                let file_name = format!("./data/{}.csv",pid);
                let path = Path::new(&file_name);
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(path)
                    .expect("Failed to open file");

                writeln!(file, "{}", rss_value)
                    .expect("Failed to write to file");
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}
