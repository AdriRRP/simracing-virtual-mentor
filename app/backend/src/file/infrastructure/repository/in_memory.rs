use crate::file::domain::file::File;
use crate::file::domain::files::Files;
use crate::file::domain::repository::Repository;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};

pub struct InMemory {
    files: Arc<Mutex<Vec<File>>>,
}

impl Default for InMemory {
    fn default() -> Self {
        Self {
            files: Arc::new(Mutex::new(Vec::default())),
        }
    }
}

#[async_trait]
impl Repository for InMemory {
    async fn create(&self, file: File) {
        let mut files_guard = self.files.lock().unwrap();
        files_guard.push(file);
        drop(files_guard);
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let mut files_guard = self.files.lock().unwrap();
        let i = files_guard
            .iter()
            .position(|file| file.id == id)
            .ok_or(format!("No laps with {id} found."))?;
        files_guard.remove(i);
        drop(files_guard);
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<File>, String> {
        let files_guard = self.files.lock().unwrap();
        let result = files_guard.iter().find(|file| file.id == id).cloned();
        drop(files_guard);
        Ok(result)
    }

    async fn find_by_criteria(&self, _criteria: &String) -> Result<Option<Files>, String> {
        let files_guard = self.files.lock().unwrap();
        let files: Files = files_guard.clone().into();
        drop(files_guard);
        let opt_files: Option<Files> = if files.len() == 0 { None } else { Some(files) };
        Ok(opt_files)
    }
}
