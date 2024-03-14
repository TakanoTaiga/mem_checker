use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    loop {
        let output = Command::new("pmap")
            .args(["-x", "54875"])
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines = output_str.lines().skip(1);

        let mut rss_map: HashMap<String, u32> = HashMap::new();

        for line in lines {
            let columns: Vec<&str> = line.split_whitespace().collect();

            let rss = columns.get(2).and_then(|&s| s.parse::<u32>().ok()).unwrap_or(0);
            let name = columns.get(5).unwrap_or(&"");

            if rss == 0 || !name.contains(".so") {
                continue;
            }

            *rss_map.entry(name.to_string()).or_insert(0) += rss;
        }

        for (name, total_rss) in rss_map {
            let file_name = format!("./data/{}.csv",name);
                let path = Path::new(&file_name);
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(path)
                    .expect("Failed to open file");

                writeln!(file, "{}",total_rss)
                    .expect("Failed to write to file");
        }
        thread::sleep(Duration::from_secs(1));
    }
}
