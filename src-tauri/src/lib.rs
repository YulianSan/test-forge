use base64::{
    engine::{self, general_purpose},
    Engine,
};
use std::{
    fs::{read, read_dir},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
enum ExtensionImage {
    PNG,
    JPG,
    JPEG,
    BMP,
    EWEB,
}

struct W<'a>(&'a str);

impl<'a> TryFrom<W<'a>> for ExtensionImage {
    type Error = String;

    fn try_from(value: W<'a>) -> Result<Self, Self::Error> {
        match value {
            W("png") => Ok(ExtensionImage::PNG),
            W("jpg") => Ok(ExtensionImage::JPG),
            _ => Err("Extension not supported".to_string()),
        }
    }
}

impl Into<String> for ExtensionImage {
    fn into(self) -> String {
        match self {
            ExtensionImage::PNG => "png".to_string(),
            ExtensionImage::JPG => "jpg".to_string(),
            ExtensionImage::JPEG => "jpeg".to_string(),
            ExtensionImage::BMP => "bmp".to_string(),
            ExtensionImage::EWEB => "eweb".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Image {
    path: String,
    url: String,
    name: String,
    extension: ExtensionImage,
}

impl Image {
    fn new(path: String, name: String, extension: ExtensionImage, url: Option<String>) -> Self {
        Self {
            path,
            url: url.unwrap_or("".to_string()),
            name,
            extension,
        }
    }

    fn to_base64(&mut self) {
        let meta: String = self.extension.clone().into();

        let file = read(self.path.clone()).unwrap();
        let encoded = general_purpose::STANDARD.encode(file);

        self.url = format!("data:image/{};base64,{}", meta, encoded);
    }
}

type Images = Vec<Image>;

#[tauri::command]
fn get_images_dir(str_path: &str) -> Result<Images, String> {
    let dir_path = Path::new(str_path);
    let dir = read_dir(dir_path).map_err(|e| e.to_string())?;
    let mut images = vec![];

    for file in dir {
        let file = match file {
            Ok(file) => file,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let path = file.path();

        if path.is_dir() {
            get_images_dir(file.path().join(dir_path).to_str().unwrap())?;
        } else {
            let extension = path.extension().and_then(std::ffi::OsStr::to_str);

            let mut image = Image::new(
                file.path().into_os_string().into_string().unwrap(),
                file.file_name().into_string().unwrap(),
                ExtensionImage::try_from(W(extension.unwrap())).unwrap(),
                None,
            );

            image.to_base64();

            images.push(image);
        }
    }

    Ok(images)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_images_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
