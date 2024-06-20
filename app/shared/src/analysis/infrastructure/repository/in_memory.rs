use crate::analysis::domain::analyses::Analyses;
use crate::analysis::domain::analysis::header::Header;
use crate::analysis::domain::analysis::Analysis;
use crate::analysis::domain::repository::Repository;
use crate::common::domain::criteria::filters::Filters;
use crate::common::domain::criteria::Criteria;

use crate::analysis::domain::analysis::headers::Headers;
use async_trait::async_trait;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// An in-memory repository implementation for asynchronous operations.
pub struct InMemory {
    analyses: Arc<Mutex<Vec<Analysis>>>,
}

impl Default for InMemory {
    /// Creates a new `InMemory` instance with an empty list of analyses.
    fn default() -> Self {
        Self {
            analyses: Arc::new(Mutex::new(Vec::default())),
        }
    }
}

#[async_trait]
impl Repository for InMemory {
    /// Creates a new analysis and adds it to the in-memory storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the analysis creation fails.
    async fn create(&self, analysis: Analysis) -> Result<(), String> {
        self.analyses
            .lock()
            .map_err(|e| format!("{e}"))?
            .push(analysis);
        Ok(())
    }

    /// Deletes an analysis with the specified ID from the in-memory storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the analysis deletion fails or if no analysis with the specified ID is found.
    async fn delete(&self, id: &Uuid) -> Result<(), String> {
        let mut analyses_guard = self.analyses.lock().map_err(|e| format!("{e}"))?;
        let i = analyses_guard
            .iter()
            .position(|analysis| analysis.header.id == *id)
            .ok_or_else(|| format!("No analysis with ID {id} found."))?;

        analyses_guard.remove(i);
        drop(analyses_guard);
        Ok(())
    }

    /// Finds an analysis with the specified ID in the in-memory storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the analysis retrieval fails or if no analysis with the specified ID is found.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Analysis>, String> {
        let analyses_ward = self.analyses.lock().map_err(|e| format!("{e}"))?;
        let result = analyses_ward
            .iter()
            .find(|analysis| analysis.header.id == *id)
            .cloned();
        drop(analyses_ward);
        Ok(result)
    }

    /// Finds analyses based on the specified criteria in the in-memory storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the analysis retrieval fails.
    async fn find_by_criteria(&self, criteria: &Criteria) -> Result<Option<Analyses>, String> {
        let analyses_guard = self.analyses.lock().map_err(|e| format!("{e}"))?;
        let mut analyses: Analyses = criteria.filters().map_or_else(
            || analyses_guard.clone().into(),
            |filters| {
                analyses_guard
                    .iter()
                    .filter(|&a| Self::pass_criteria_filters(&filters, a))
                    .map(Clone::clone)
                    .collect::<Vec<Analysis>>()
                    .into()
            },
        );

        drop(analyses_guard);

        if let Some(order) = criteria.order() {
            analyses.sort_by(|a, b| {
                if let (Ok(a), Ok(b)) = (
                    Self::map_header_field(&a.header, order.by().field_name()),
                    Self::map_header_field(&b.header, order.by().field_name()),
                ) {
                    order.apply(&a, &b)
                } else {
                    Ordering::Equal
                }
            });
        }

        let opt_analyses: Option<Analyses> = if analyses.len() == 0 {
            None
        } else {
            Some(analyses)
        };

        Ok(opt_analyses)
    }

    async fn find_header_by_id(&self, id: &Uuid) -> Result<Option<Header>, String> {
        let analyses_ward = self.analyses.lock().map_err(|e| format!("{e}"))?;
        let result = analyses_ward
            .iter()
            .find(|&h| h.header.id == *id)
            .cloned()
            .map(|a| a.header);
        drop(analyses_ward);
        Ok(result)
    }

    async fn find_header_by_criteria(
        &self,
        criteria: &Criteria,
    ) -> Result<Option<Headers>, String> {
        let mut headers: Headers = {
            let analyses_guard = self.analyses.lock().map_err(|e| format!("{e}"))?;

            let analyses: Analyses = criteria.filters().map_or_else(
                || analyses_guard.clone().into(),
                |filters| {
                    analyses_guard
                        .iter()
                        .filter(|&a| Self::pass_criteria_filters(&filters, a))
                        .cloned()
                        .collect()
                },
            );

            drop(analyses_guard);

            analyses.iter().map(|a| a.header.clone()).collect()
        };

        if let Some(order) = criteria.order() {
            headers.sort_by(|a, b| {
                if let (Ok(a), Ok(b)) = (
                    Self::map_header_field(a, order.by().field_name()),
                    Self::map_header_field(b, order.by().field_name()),
                ) {
                    order.apply(&a, &b)
                } else {
                    Ordering::Equal
                }
            });
        }

        let opt_headers: Option<Headers> = if headers.is_empty() {
            None
        } else {
            Some(headers)
        };

        Ok(opt_headers)
    }
}

impl InMemory {
    fn pass_criteria_filters(filters: &Filters, analysis: &Analysis) -> bool {
        filters.iter().all(|filter| {
            if let Ok(field_value) = Self::map_header_field(&analysis.header, filter.field().name())
            {
                return filter
                    .condition()
                    .apply(field_value.as_ref(), filter.value().get());
            };
            false
        })
    }

    fn map_header_field(header: &Header, field: &str) -> Result<String, String> {
        match field {
            "id" => Ok(header.id.to_string()),
            "name" => Ok(header.name.clone()),
            "circuit" => Ok(header.circuit.clone()),
            "date" => Ok(header.date.to_string()),
            _ => Err(format!("`{field}` field does not exists in analysis")),
        }
    }
}
