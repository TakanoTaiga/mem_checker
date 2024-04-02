use std::process::{Command, Stdio};
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::path::Path;

fn main() {
    let _ = create_dir_all("data");
    loop {
        let output = Command::new("ps")
            .args(["aux"])
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8_lossy(&output.stdout);

        for line in output_str.lines().skip(1) {
            if line.contains("ps aux"){ continue; }
            if line.contains("vscode"){ continue; }
            if line.contains("libexec"){ continue; }
            if line.contains("sleep 1"){ continue; }
            if line.contains("auto_ps"){ continue; }
            if line.contains(".rustup/toolchains"){ continue; }
            if line.contains("bash"){ continue; }
            if line.contains("/bin/sh"){ continue; }
            if line.contains("/usr/"){ continue; }
            if line.contains("sshd: "){ continue; }
            if line.contains("/snap/"){ continue; }

            let columns: Vec<&str> = line.split_whitespace().collect();
            let user = columns.get(0).unwrap_or(&"");
            let pid = columns.get(1).unwrap_or(&"");

            if *user == "taiga" {
                let rss_str = columns.get(5).unwrap_or(&"0");
                let rss_value: i32 = rss_str.parse().unwrap_or(0);

                let file_name = format!("./data/{}.csv",pid);
                let path = Path::new(&file_name);
                if !path.is_file() {
                    let datafile_path = Path::new("./p_list.csv");
                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open(datafile_path)
                        .expect("Failed to open file");

                    writeln!(file, "{}", line)
                        .expect("Failed to write to file");
                }
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
