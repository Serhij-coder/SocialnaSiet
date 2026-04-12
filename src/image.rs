use std::env;

use base64::{engine::general_purpose, *};
use uuid::Uuid;

pub async fn save_image(b64_image: String) -> Result<String, String> {
    let image: Vec<u8> = match general_purpose::STANDARD.decode(b64_image) {
        Ok(img_bytes) => img_bytes,
        Err(err) => return Err(format!("Error decoding image: {}", err)),
    };

    let new_id = Uuid::new_v4().to_string();
    let file_name = new_id;

    let data_dir = env::var("DATA_DIR").unwrap();
    let path = std::path::Path::new(&data_dir).join(&file_name);

    match tokio::fs::write(path, image).await {
        Ok(_) => Ok(file_name),
        Err(e) => Err(format!("Error writing file: {}", e)),
    }
}
