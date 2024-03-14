use csv::StringRecord;
use std::fs::{self};
use std::io::{self};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let data_dir = "./data";
    let optimized_dir = "./optimized";

    // optimizedディレクトリがない場合は作成
    fs::create_dir_all(optimized_dir)?;

    for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let mut rdr = csv::Reader::from_path(&path)?;
            let mut prev_record: Option<StringRecord> = None;
            let mut is_varying = false;

            for result in rdr.records() {
                let record = result?;
                if let Some(prev) = prev_record {
                    if prev != record {
                        is_varying = true;
                        break;
                    }
                }
                prev_record = Some(record);
            }

            if is_varying {
                let file_name = path.file_name().unwrap();
                let dest_path = format!("{}/{}", optimized_dir, file_name.to_str().unwrap());
                fs::copy(path, dest_path)?;
            }
        }
    }

    Ok(())
}
