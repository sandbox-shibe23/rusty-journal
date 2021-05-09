use chrono::{serde::ts_seconds, DateTIme, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: Datetime<Utc> = Utc::now();
        Task {text, created_at}
    }
}

use std::io::Result;
use std::path::PathBuf;

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    
}