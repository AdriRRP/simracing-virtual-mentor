macro_rules! num_from_le {
    // Extracts numeric data from a little-endian byte slice.
    // Optionally performs conversion to another type.
    (
        $bytes:expr,            // Byte slice containing the data.
        $start:expr,            // Start index in the byte slice.
        $end:expr,              // End index in the byte slice.
        $ty:ty,                 // Type of the numeric data to extract.
        $err_enum:ty,           // Error enum type for error variants.
        $err_variant:ident      // Error variant for conversion errors.
        $(, $try_from_ty:ty)?  // Optional type for additional conversion.
    ) => {
        {
            // Extract the value from the byte slice and handle errors.
            let value = <$ty>::from_le_bytes($bytes[$start..$end].try_into().map_err(|e| {
                // Create an error variant with a descriptive error message.
                <$err_enum>::$err_variant(String::from(format!(
                    "{} cannot be extracted from bytes &[{}..{}] -> {:?}: {}",
                    stringify!($ty),
                    $start,
                    $end,
                    &$bytes[$start..$end],
                    e
                )))
            })?);

            // Optionally perform additional type conversion.
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
    // Extracts string data from a little-endian byte slice.
    // Converts it into a fixed-size array of characters.
    (
        $bytes:expr,            // Byte slice containing the data.
        $start:expr,            // Start index in the byte slice.
        $size:expr,             // Size of the string data.
        $err_enum:ty,           // Error enum type for error variants.
        $err_variant:ident      // Error variant for conversion errors.
    ) => {{
        // Convert the string data into a fixed-size array of characters.
        String::from_utf8_lossy(&$bytes[$start..($start + $size)])
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|e| {
                // Create an error variant with a descriptive error message.
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
