use crate::common::domain::criteria::filters::Filters;
use crate::file::domain::file::File;
use crate::file::domain::files::Files;
use crate::file::domain::repository::Repository;
use std::cmp::Ordering;

use crate::common::domain::criteria::Criteria;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

/// An in-memory implementation of the file repository.
#[derive(Debug)]
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
    async fn create(&self, file: File) -> Result<(), String> {
        let mut files_guard = self.files.lock().map_err(|e| format!("{e}"))?;
        files_guard.push(file);
        drop(files_guard);
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let mut files_guard = self.files.lock().map_err(|e| format!("{e}"))?;
        let i = files_guard
            .iter()
            .position(|file| file.id == id)
            .ok_or(format!("No files with {id} found."))?;
        files_guard.remove(i);
        drop(files_guard);
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<File>, String> {
        let files_guard = self.files.lock().map_err(|e| format!("{e}"))?;
        let result = files_guard.iter().find(|file| file.id == id).cloned();
        drop(files_guard);
        Ok(result)
    }

    async fn find_by_criteria(&self, criteria: &Criteria) -> Result<Option<Files>, String> {
        let files_guard = self.files.lock().map_err(|e| format!("{e}"))?;

        let mut files: Files = criteria.filters().map_or_else(
            || files_guard.clone().into(),
            |filters| {
                files_guard
                    .iter()
                    .filter(|&f| Self::pass_criteria_filters(&filters, f))
                    .map(Clone::clone)
                    .collect::<Vec<File>>()
                    .into()
            },
        );

        drop(files_guard);

        if let Some(order) = criteria.order() {
            files.sort_by(|a, b| {
                if let (Ok(a), Ok(b)) = (
                    Self::map_file_field(a, order.by().field_name()),
                    Self::map_file_field(b, order.by().field_name()),
                ) {
                    order.apply(&a, &b)
                } else {
                    Ordering::Equal
                }
            });
        }

        let opt_files: Option<Files> = if files.len() == 0 { None } else { Some(files) };

        Ok(opt_files)
    }

    async fn validate(&self, id: &str) -> Result<(), String> {
        let mut files_guard = self.files.lock().map_err(|e| format!("{e}"))?;
        if let Some(f) = files_guard.iter_mut().find(|f| f.id == id) {
            f.success();
        }
        drop(files_guard);
        Ok(())
    }
    async fn mark_as_error(&self, id: &str, msg: &str) -> Result<(), String> {
        let mut files_guard = self.files.lock().map_err(|e| format!("{e}"))?;
        if let Some(f) = files_guard.iter_mut().find(|f| f.id == id) {
            f.fail(msg);
        }
        drop(files_guard);
        Ok(())
    }
}

impl InMemory {
    fn pass_criteria_filters(filters: &Filters, file: &File) -> bool {
        filters.iter().all(|filter| {
            if let Ok(field_value) = Self::map_file_field(file, filter.field().name()) {
                return filter
                    .condition()
                    .apply(field_value.as_ref(), filter.value().get());
            };
            false
        })
    }

    fn map_file_field(file: &File, field: &str) -> Result<String, String> {
        match field {
            "id" => Ok(file.id.clone()),
            "name" => Ok(file.name.clone()),
            "created_on" => Ok(file.created_on.clone().to_string()),
            "status" => serde_json::to_string(&file.status).map_err(|e| format!("{e}")),
            _ => Err(format!("`{field}` field does not exists in file")),
        }
    }
}
