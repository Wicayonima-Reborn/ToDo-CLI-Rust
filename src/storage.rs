use crate::task::Task;
use std::fs;
use std::path::Path;

const FILE_PATH: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Gagal serialisasi");
    fs::write(FILE_PATH, data).expect("Gagal menulis file");
}