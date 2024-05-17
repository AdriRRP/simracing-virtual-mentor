use crate::ibt::domain::file::File;

use axum::extract::Multipart;
use axum::http::StatusCode;

use futures_util::TryFutureExt;

use std::io::Cursor;

pub async fn upload_file(mut multipart: Multipart) -> Result<StatusCode, (StatusCode, String)> {
    let mut body_bytes = Vec::<u8>::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let byte_slice = field
            .bytes()
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            .await?;

        body_bytes.extend_from_slice(&byte_slice);
    }

    let cursor = Cursor::new(body_bytes);

    tokio::spawn(async move {
        run_command(cursor);
    });

    Ok(StatusCode::ACCEPTED)
}

fn run_command(reader: Cursor<Vec<u8>>) {
    let mut reader = reader;
    let file = File::from_reader(&mut reader, &None);
    println!("{:?}", file.unwrap().disk_header);
}
