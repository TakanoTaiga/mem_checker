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
            .args(["aux" , "-T"])
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8_lossy(&output.stdout);

        for line in output_str.lines() {
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
            if !line.contains("taiga"){ continue; }

            let columns: Vec<&str> = line.split_whitespace().collect();
            let pid = columns.get(1).unwrap_or(&"");
            let thread_pid = columns.get(2).unwrap_or(&"");
            let rss = columns.get(6).unwrap_or(&"");

            let file_name = format!("./data/{}_{}.csv", pid, thread_pid);
            let path = Path::new(&file_name);
            let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(path)
                    .expect("Failed to open file");

            writeln!(file, "{}", rss)
                .expect("Failed to write to file");
        }

        thread::sleep(Duration::from_secs(1));
    }
}
