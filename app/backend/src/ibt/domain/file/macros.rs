macro_rules! num_from_le {
    ($bytes:expr, $start:expr, $end:expr, $ty:ty, $err_enum:ty, $err_variant:ident $(, $try_from_ty:ty)?) => {
        {
            let value = <$ty>::from_le_bytes($bytes[$start..$end].try_into().map_err(|e| {
                <$err_enum>::$err_variant(String::from(format!(
                    "{} cannot be extracted from bytes &[{}..{}] -> {:?}: {}",
                    stringify!($ty),
                    $start,
                    $end,
                    &$bytes[$start..$end],
                    e
                )))
            })?);

            $(
                let value = match <$try_from_ty>::try_from(value) {
                    Ok(val) => val,
                    Err(e) => return Err(<$err_enum>::$err_variant(String::from(format!(
                    "{} of type {} cannot be converted converted to type {}: {}",
                    value,
                    stringify!($ty),
                    stringify!($try_from_ty),
                    e
                )))),
                };
            )?

            value
        }
    };
}
pub(crate) use num_from_le;

macro_rules! str_from_le {
    ($bytes:expr, $start:expr, $size:expr, $err_enum:ty, $err_variant:ident) => {{
        String::from_utf8_lossy(&$bytes[$start..($start + $size)])
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|e| {
                <$err_enum>::$err_variant(format!(
                    "[char; {}] cannot be extracted from bytes &[{}..{}] -> {:?}: {:?}",
                    $size,
                    $start,
                    $start + $size,
                    &$bytes[$start..($start + $size)],
                    e
                ))
            })?
    }};
}
pub(crate) use str_from_le;
