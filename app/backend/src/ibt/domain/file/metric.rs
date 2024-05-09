use crate::ibt::domain::file::var_header::VarHeader;
use crate::ibt::domain::file::var_value::VarValue;

#[derive(Debug)]
pub struct Metric {
    header: VarHeader,
    var_values: Vec<VarValue>,
    index: usize,
}

impl Metric {}
