use std::{env, fmt::format};

use base64::{engine::general_purpose, *};
use uuid::Uuid;

use std::path::PathBuf;

pub enum ImageType {
    Pfp,
    Topic,
    Chat { topic_name: String },
}

/// Save image to disc and return it name or error message
pub async fn save_image(b64_image: String, image_type: ImageType) -> Result<String, String> {
    let image: Vec<u8> = match general_purpose::STANDARD.decode(b64_image) {
        Ok(img_bytes) => img_bytes,
        Err(err) => return Err(format!("Error decoding image: {}", err)),
    };

    let new_id = Uuid::new_v4().to_string();
    let file_name = new_id;

    let data_dir_raw = env::var("DATA_DIR").unwrap();
    let data_dir = data_dir_raw
        .strip_suffix("/")
        .unwrap_or(data_dir_raw.as_str());

    let data_dir = match image_type {
        ImageType::Pfp => format!("{}/{}/", data_dir, "pfp"),
        ImageType::Topic => format!("{}/{}/", data_dir, "topic"),
        ImageType::Chat { topic_name } => format!("{}/{}/{}/", data_dir, "chat", topic_name),
    };

    let path = std::path::Path::new(&data_dir).join(&file_name);

    match tokio::fs::write(path, image).await {
        Ok(_) => Ok(file_name),
        Err(e) => Err(format!("Error writing file: {}", e)),
    }
}
