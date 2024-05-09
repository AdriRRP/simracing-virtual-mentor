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

    let ibt_file = IbtFile::from_stream(&mut f)
        .map_err(|e| io::Error::new(ErrorKind::Other, format!("{e}")))?;

    //let vars: Vec<(VarHeader, &VarValue)> = ibt_file
    //    .var_headers
    //    .zip(ibt_file.var_values.iter())
    //    .collect();
    //
    //for (var_header, var_value) in vars {
    //    println!(
    //        "{}: {}. Type: {}. Unit: {}. Count: {}. CaT: {}. Offset: {}. Value = {:?})",
    //        var_header.name(),
    //        var_header.description(),
    //        var_header.var_type,
    //        var_header.unit(),
    //        var_header.count,
    //        var_header.count_as_time,
    //        var_header.offset,
    //        var_value,
    //    );
    //}

    //for var_header in ibt_file.var_headers {
    //    println!(
    //        "{}: {}. Type: {}. Unit: {}. Count: {}. CaT: {}. Offset: {})",
    //        var_header.name(),
    //        var_header.description(),
    //        var_header.var_type,
    //        var_header.unit(),
    //        var_header.count,
    //        var_header.count_as_time,
    //        var_header.offset,
    //    );
    //}

    println!("{:?}", ibt_file);

    Ok(())
}
