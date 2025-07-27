use crate::config::parquet::ParquetDataset;
use crate::{error::AppResult, get_parquet_datasets};
use bytes::Bytes;
use polars::prelude::*;
use salvo::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

fn tile_bbox(z: u32, x: u32, y: u32) -> (f64, f64, f64, f64) {
    let n = 2f64.powi(z as i32);
    let lon_deg_min = x as f64 / n * 360.0 - 180.0;
    let lon_deg_max = (x as f64 + 1.0) / n * 360.0 - 180.0;
    let lat_rad_min = ((y as f64 + 1.0) / n * std::f64::consts::PI * -2.0)
        .sinh()
        .atan();
    let lat_deg_min = lat_rad_min.to_degrees();
    let lat_rad_max = (y as f64 / n * std::f64::consts::PI * -2.0).sinh().atan();
    let lat_deg_max = lat_rad_max.to_degrees();
    (lon_deg_min, lat_deg_min, lon_deg_max, lat_deg_max)
}

fn collect_parquet_files(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_parquet_files(&path, files)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("parquet") {
            files.push(path);
        }
    }
    Ok(())
}

async fn generate_tile(dataset: &ParquetDataset, z: u32, x: u32, y: u32) -> AppResult<Bytes> {
    let (min_lon, min_lat, max_lon, max_lat) = tile_bbox(z, x, y);
    let mut points: Vec<(f64, f64)> = Vec::new();

    let mut files = Vec::new();
    collect_parquet_files(Path::new(&dataset.directory), &mut files)?;

    for file in files {
        let lf = LazyFrame::scan_parquet(file.to_str().unwrap(), Default::default())?;
        let df = lf
            .filter(
                col(&dataset.lon_col)
                    .gt_eq(lit(min_lon))
                    .and(col(&dataset.lon_col).lt_eq(lit(max_lon)))
                    .and(col(&dataset.lat_col).gt_eq(lit(min_lat)))
                    .and(col(&dataset.lat_col).lt_eq(lit(max_lat))),
            )
            .select([col(&dataset.lon_col), col(&dataset.lat_col)])
            .collect()?;

        let lon_series = df.column(&dataset.lon_col)?.f64()?;
        let lat_series = df.column(&dataset.lat_col)?.f64()?;

        for i in 0..df.height() {
            let lon = lon_series.get(i).unwrap_or(0.0);
            let lat = lat_series.get(i).unwrap_or(0.0);
            points.push((lon, lat));
        }
    }

    // TODO: encode points into MVT. This is a placeholder.
    Ok(Bytes::from(vec![]))
}

#[handler]
pub async fn get_parquet_tile(req: &mut Request, res: &mut Response) -> AppResult<()> {
    res.headers_mut().insert(
        "content-type",
        "application/x-protobuf;type=mapbox-vector".parse().unwrap(),
    );

    let dataset_name = req.param::<String>("dataset").unwrap_or_default();
    let x = req.param::<u32>("x").unwrap_or(0);
    let y = req.param::<u32>("y").unwrap_or(0);
    let z = req.param::<u32>("z").unwrap_or(0);

    let datasets = get_parquet_datasets().await.read().await;
    let Some(ds) = datasets.iter().find(|d| d.name == dataset_name) else {
        res.body(salvo::http::ResBody::Once(Bytes::new()));
        return Ok(());
    };

    let tile = generate_tile(ds, z, x, y)
        .await
        .unwrap_or_else(|_| Bytes::new());
    res.body(salvo::http::ResBody::Once(tile));
    Ok(())
}
