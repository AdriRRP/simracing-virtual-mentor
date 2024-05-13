use crate::ibt::domain::file::var_filter::VarFilter;
use crate::ibt::domain::file::var_header::VarHeader;
use crate::ibt::domain::file::var_value::primitive::Primitive;
use crate::ibt::domain::file::var_value::VarValue;
use crate::ibt::domain::file::File as IbtFile;

use std::fs::File as StdFile;
use std::io::ErrorKind;
use std::{env, io};

pub mod ibt {
    pub mod domain {
        pub mod file;
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let Some(path) = args.get(1) else {
        panic!("First argument `path` not found")
    };

    let filter = args
        .get(2)
        .map(|filter_str| VarFilter::new(filter_str.split(',').map(ToString::to_string).collect()));

    let mut f = StdFile::open(path)?;

    let ibt_file = IbtFile::from_reader(&mut f, &filter)
        .map_err(|e| io::Error::new(ErrorKind::Other, format!("{e}")))?;

    // PRINT CSV VAR HEADERS
    //println!("name,description,var_type,unit,count,count_as_time,offset");
    //ibt_file.metrics.iter().map(|m| &m.var_header).for_each(|var_header| {
    //    println!(
    //        "{},{},{},{},{},{},{}",
    //        var_header.name(),
    //        var_header.description(),
    //        var_header.var_type,
    //        var_header.unit(),
    //        var_header.count,
    //        var_header.count_as_time,
    //        var_header.offset
    //    );
    //});

    // PRINT CSV VAR VALUES
    let header = ibt_file
        .metrics
        .iter()
        .map(|m| &m.var_header)
        .map(VarHeader::name)
        .collect::<Vec<String>>()
        .join(",");

    println!("{header}");

    (0..ibt_file.metrics.first().unwrap().var_values.len()).for_each(|i| {
        let row = ibt_file
            .metrics
            .iter()
            .map(|metric| match metric.var_values.get(i).unwrap() {
                VarValue::Single(prim) => primitive_to_string(prim),
                VarValue::Array(av) => {
                    let str_values: Vec<String> = av.iter().map(primitive_to_string).collect();
                    format!("\"[{}]\"", str_values.join(" "))
                }
            })
            .collect::<Vec<String>>()
            .join(",");
        println!("{row}");
    });

    Ok(())
}

fn primitive_to_string(primitive: &Primitive) -> String {
    match primitive {
        Primitive::Bool(pv) => pv.to_string(),
        Primitive::Char(pv) => pv.to_string(),
        Primitive::Int(pv) => pv.to_string(),
        Primitive::BitField(pv) => pv.to_string(),
        Primitive::Float(pv) => pv.to_string(),
        Primitive::Double(pv) => pv.to_string(),
    }
}
