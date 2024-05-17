use crate::ibt::domain::file::from_reader;
use crate::ibt::domain::file::header::Header;
use crate::ibt::domain::file::metric::Metric;
use crate::ibt::domain::file::var_filter::VarFilter;
use crate::ibt::domain::file::var_headers::VarHeaders;

use std::io::{Read, Seek};
use std::ops::Deref;

#[derive(PartialEq, Debug)]
pub struct Metrics {
    metrics: Vec<Metric>,
}

impl Metrics {
    /// # Errors
    ///
    /// Will return `Err` if max by tick count `header.var_buffers` can't be retrieved,
    /// or if `Metric::from_reader` fails
    #[allow(clippy::similar_names)]
    pub fn from_reader<ReadSeek: Read + Seek>(
        reader: &mut ReadSeek,
        header: &Header,
        filter: &Option<VarFilter>,
    ) -> Result<Self, from_reader::Error> {
        // Headers of all metrics
        let mut var_headers = VarHeaders::from_reader(reader, header)?;

        // Size of a sample of values of all headers
        let var_block_size = var_headers
            .iter()
            .fold(0usize, |acc, e| acc + e.var_type.byte_size() * e.count)
            as u64;

        // Filter headers
        if let Some(var_filter) = filter {
            var_headers.retain(|var_header| var_filter.allow(var_header));
        }
        

        // TODO: Async parsing
        //let metrics_tasks: Vec<JoinHandle<Result<Metric, from_reader::Error>>> = var_headers
        //    .iter()
        //    .map(|var_header| {
        //        // Pick the buffer with the highest tick_count
        //        let current_buffer = header
        //            .var_buffers
        //            .iter()
        //            .max_by_key(|b| b.tick_count)
        //            .ok_or_else(|| {
        //                from_reader::Error::Reading(
        //                    "Can't get the buffer with the highest tick_count".to_string(),
        //                )
        //            });
        //        task::spawn(async move {
        //            let current_buffer = current_buffer?;
        //            Metric::from_reader(reader, var_header, current_buffer.offset, var_block_size)
        //        })
        //    })
        //    .collect();
        //let results = join_all(metrics_tasks).await;
        //let results: Result<Vec<Metric>, from_reader::Error> = results
        //    .into_iter()
        //    .map(|result| result.map_err(|e| from_reader::Error::Reading(format!("{e}")))?)
        //    .collect();
        //
        //results.map(|metrics| Self { metrics })
        
        // Result implements FromIterator, so you can move the Result outside and iterators will
        // take care of the rest (including stopping iteration if an error is found).
        let metrics_result: Result<Vec<Metric>, from_reader::Error> = var_headers
            .iter()
            .map(|var_header| {
                // Pick the buffer with the highest tick_count
                let current_buffer = header
                    .var_buffers
                    .iter()
                    .max_by_key(|b| b.tick_count)
                    .ok_or_else(|| {
                        from_reader::Error::Reading(
                            "Can't get the buffer with the highest tick_count".to_string(),
                        )
                    })?;
                Metric::from_reader(reader, var_header, current_buffer.offset, var_block_size)
            })
            .collect();

        metrics_result.map(|metrics| Self { metrics })
    }
}

impl Deref for Metrics {
    type Target = Vec<Metric>;

    fn deref(&self) -> &Self::Target {
        &self.metrics
    }
}
