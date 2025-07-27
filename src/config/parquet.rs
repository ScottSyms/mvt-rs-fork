use serde::Deserialize;
use std::fs;
use crate::error::AppResult;

#[derive(Debug, Clone, Deserialize)]
pub struct ParquetDataset {
    pub name: String,
    pub directory: String,
    pub lat_col: String,
    pub lon_col: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParquetConfig {
    pub datasets: Vec<ParquetDataset>,
}

pub fn load_parquet_config(config_dir: &str) -> AppResult<Vec<ParquetDataset>> {
    let file_path = format!("{}/parquet.json", config_dir);
    let data = fs::read_to_string(&file_path)?;
    let cfg: ParquetConfig = serde_json::from_str(&data)?;
    Ok(cfg.datasets)
}
