pub mod ibt {
    pub mod domain {
        pub mod file;
    }
}

use crate::ibt::domain::file::File as IbtFile;

use std::fs::File as StdFile;
use std::io;
use std::io::ErrorKind;

fn main() -> io::Result<()> {
    let mut f = StdFile::open("/Users/adrianramos/Downloads/telemetry.ibt")?;

    let ibt_file = IbtFile::from_reader(&mut f)
        .map_err(|e| io::Error::new(ErrorKind::Other, format!("{e}")))?;

    println!(
        "Theoretical number of metrics: {}",
        ibt_file.header.num_vars
    );
    println!("Number of metrics: {}", ibt_file.metrics.len());
    ibt_file.metrics.iter().for_each(|metric| {
        let var_header = &metric.var_header;
        println!(
            "{}: {}. Type: {}. Unit: {}. Count: {}. CaT: {}. Offset: {}. Total: {})",
            var_header.name(),
            var_header.description(),
            var_header.var_type,
            var_header.unit(),
            var_header.count,
            var_header.count_as_time,
            var_header.offset,
            metric.len()
        );
        metric.iter().for_each(|v| println!("{:?}", v));
    });

    //metric.iter().for_each(|v| println!("{:?}", v));

    Ok(())
}
