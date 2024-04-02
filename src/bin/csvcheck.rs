use plotters::prelude::*;
use csv::StringRecord;
use std::fs::{self};
use std::io::{self};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let data_dir = "./data";
    let optimized_dir = "./optimized";
    let graph_dir = "./graphs";

    fs::create_dir_all(optimized_dir)?;
    fs::create_dir_all(graph_dir)?;

    for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let mut rdr = csv::Reader::from_path(&path)?;
            let mut values = Vec::new(); // 数値を格納するベクタ
            let mut is_varying = false;
            let mut prev_record: Option<StringRecord> = None;
            let mut first: Option<i32> = None;

            for result in rdr.records() {
                let record = result?;
                if let Some(prev) = &prev_record {
                    if prev != &record {
                        is_varying = true;
                        break; // 変化を検出したらループを抜ける
                    }
                }
                prev_record = Some(record.clone());

                if first.is_none() {
                    if let Ok(val) = record[0].parse::<i32>() {
                        first = Some(val);
                    }
                }
            }

            if let Some(first_val) = first {
                // 再度Readerを初期化して、全ての値を処理
                let mut rdr = csv::Reader::from_path(&path)?;
                for result in rdr.records() {
                    let record = result?;
                    if let Ok(val) = record[0].parse::<i32>() {
                        values.push(val - first_val);
                    }
                }
            }

            if is_varying {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let dest_path = format!("{}/{}", optimized_dir, file_name);
                fs::copy(path, &dest_path)?;

                // グラフをPNGファイルとして保存
                let graph_path = format!("{}/{}.png", graph_dir, file_name);
                let _ = save_graph(&graph_path, &values);
            }
        }
    }

    Ok(())
}

fn save_graph(file_path: &str, values: &[i32]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let min = *values.iter().min().unwrap_or(&0);
    let max = *values.iter().max().unwrap_or(&0);
    let mut chart = ChartBuilder::on(&root)
        .caption("Data Graph", ("sans-serif", 50))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0..values.len(), min..max)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        values.iter().enumerate().map(|(x, y)| (x, *y)),
        &RED,
    ))?;
    root.present()?;
    Ok(())
}
