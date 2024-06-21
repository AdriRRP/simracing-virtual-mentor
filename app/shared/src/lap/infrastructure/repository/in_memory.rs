use crate::common::domain::criteria::filters::Filters;
use crate::common::domain::criteria::Criteria;
use crate::lap::domain::lap::header::Header;
use crate::lap::domain::lap::headers::Headers;
use crate::lap::domain::lap::Lap;
use crate::lap::domain::laps::Laps;
use crate::lap::domain::repository::Repository;

use async_trait::async_trait;
use std::cmp::Ordering;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// A repository implementation storing laps in memory.
#[derive(Debug)]
pub struct InMemory {
    laps: Arc<Mutex<Vec<Lap>>>,
}

impl Default for InMemory {
    /// Creates a new `InMemory` repository with an empty laps collection.
    fn default() -> Self {
        Self {
            laps: Arc::new(Mutex::new(Vec::default())),
        }
    }
}

#[async_trait]
impl Repository for InMemory {
    /// Asynchronously creates laps in memory.
    async fn create(&self, laps: Laps) -> Result<(), String> {
        let mut laps_guard = self.laps.lock().map_err(|e| format!("{e}"))?;
        laps_guard.extend(laps.deref().clone());
        drop(laps_guard);
        Ok(())
    }

    /// Asynchronously deletes laps from memory by ID.
    async fn delete(&self, id: &Uuid) -> Result<(), String> {
        let mut laps_guard = self.laps.lock().map_err(|e| format!("{e}"))?;
        let i = laps_guard
            .iter()
            .position(|lap| lap.header.id == *id)
            .ok_or(format!("No lap with {id} found."))?;
        laps_guard.remove(i);
        drop(laps_guard);
        Ok(())
    }

    /// Asynchronously finds a lap by ID.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Lap>, String> {
        let laps_guard = self.laps.lock().map_err(|e| format!("{e}"))?;
        let result = laps_guard.iter().find(|lap| lap.header.id == *id).cloned();
        drop(laps_guard);
        Ok(result)
    }

    /// Asynchronously finds laps by criteria.
    async fn find_by_criteria(&self, criteria: &Criteria) -> Result<Option<Laps>, String> {
        let laps_guard = self.laps.lock().map_err(|e| format!("{e}"))?;
        let mut laps: Laps = criteria.filters().map_or_else(
            || laps_guard.clone().into(),
            |filters| {
                laps_guard
                    .iter()
                    .filter(|&l| Self::pass_criteria_filters(&filters.clone(), l))
                    .map(Clone::clone)
                    .collect::<Vec<Lap>>()
                    .into()
            },
        );

        drop(laps_guard);

        if let Some(order) = criteria.order() {
            laps.sort_by(|a, b| {
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

        let opt_laps: Option<Laps> = if laps.len() == 0 { None } else { Some(laps) };

        Ok(opt_laps)
    }

    /// Asynchronously finds the header of a lap by ID.
    async fn find_header_by_id(&self, id: &Uuid) -> Result<Option<Header>, String> {
        self.find_by_id(id)
            .await
            .map(|opt_lap| opt_lap.map(|lap| lap.header))
    }

    /// Asynchronously finds headers of laps by criteria.
    async fn find_header_by_criteria(
        &self,
        criteria: &Criteria,
    ) -> Result<Option<Headers>, String> {
        let mut headers: Vec<Header> = {
            let laps_guard = self.laps.lock().map_err(|e| format!("{e}"))?;

            let laps: Laps = criteria.filters().map_or_else(
                || laps_guard.clone().into(),
                |filters| {
                    laps_guard
                        .iter()
                        .filter(|&l| Self::pass_criteria_filters(&filters.clone(), l))
                        .cloned()
                        .collect()
                },
            );

            drop(laps_guard);

            laps.iter().map(|l| l.header.clone()).collect()
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
            Some(Headers::from(headers))
        };

        Ok(opt_headers)
    }
}

impl InMemory {
    fn pass_criteria_filters(filters: &Filters, lap: &Lap) -> bool {
        filters.iter().all(|filter| {
            if let Ok(field_value) = Self::map_header_field(&lap.header, filter.field().name()) {
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
            "driver" => Ok(header.driver.clone()),
            "category" => Ok(header.category.clone()),
            "car" => Ok(header.car.clone()),
            "number" => Ok(header.number.to_string()),
            "file_id" => Ok(header.file_id.to_string()),
            "circuit" => Ok(header.circuit.clone()),
            "time" => Ok(header.time.to_string()),
            "date" => Ok(header.date.to_string()),
            _ => Err(format!("`{field}` lap file does not exists in lap")),
        }
    }
}
