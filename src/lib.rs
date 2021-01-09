use image::{DynamicImage, GenericImageView};
use glium::texture::RawImage2d;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use Value::Object;
use glium::backend::Facade;

struct Texture {
    pub image: DynamicImage,
}

impl Texture {
    pub fn from_file(path: &str) -> Texture {
        Texture {
            image: image::open(path).unwrap(),
        }
    }

    pub fn as_raw_image_2d(&self) -> RawImage2d<u8> {
        return glium::texture::RawImage2d::from_raw_rgba_reversed(&self.image.to_rgba8().into_raw(), self.image.dimensions());
    }
}

const DEFAULT_CONFIG_PATH: &str = "texture_config.json";

/// Common structure storage with all the data to operate textures in project.
/// Encapsulates loading logic, supports lazy loading.
/// Note: by default, all config keys are String types.
/// Todo: remake it on more generic usage with any displayable type, not only string.
pub struct TextureBag {
    /// Data read by config. For now Hashmap of texture ID -> path to texture.
    /// It might change until first stable version, but interfaces won't be affected.
    config_data: HashMap<String, String>,

    /// Textures as glium::texture::Texture2d. Will extend it to other texture types if needed.
    textures: HashMap<String, glium::texture::Texture2d>,
}

impl TextureBag {
    /// Loads all textures described in config into memory
    /// # Panics
    ///
    /// Method will panic if config file is missing.
    pub fn init_eager<F>(facade: &F, config_path: Option<String>)
                         -> TextureBag where F: Facade
    {
        let path = match config_path {
            Some(path) => path,
            None => String::from(DEFAULT_CONFIG_PATH),
        };

        let file = File::open(&path).unwrap();
        let buffered_reader = BufReader::new(file);
        let file_data: Value = serde_json::from_reader(buffered_reader).unwrap();
        let loaded_textures_config = file_data.get("textures").unwrap();
        let mut converted_config: HashMap<String, String> = HashMap::new();

        match loaded_textures_config {
            Object(config) => {
                for entry in config.clone() {
                    match entry.1 {
                        Value::String(path) => {
                            converted_config.insert(entry.0, path);
                        }
                        _ => panic!("Expected texture path for texture ID {}", entry.0)
                    }
                }
            }
            _ => panic!("Invalid JSON structure. Expected 'textures' key-value object.")
        }

        let mut loaded_textures: HashMap<String, glium::texture::Texture2d> = HashMap::new();

        for texture in converted_config.clone() {
            loaded_textures.insert(
                texture.0,
                glium::texture::Texture2d::new(facade, Texture::from_file(texture.1.as_str()).as_raw_image_2d()).unwrap()
            );
        }

        TextureBag {
            config_data: converted_config,
            textures: loaded_textures
        }
    }

    /// # Panics
    ///
    /// Method will panic if config file is missing or if json format is invalid.
    pub fn init_lazy<F>(_facade: &F, config_path: Option<String>)
        -> TextureBag where F: Facade
    {
        let path = match config_path {
            Some(path) => path,
            None => String::from(DEFAULT_CONFIG_PATH),
        };

        let file = File::open(&path).unwrap();
        let buffered_reader = BufReader::new(file);
        let file_data: Value = serde_json::from_reader(buffered_reader).unwrap();
        let loaded_textures_config = file_data.get("textures").unwrap();
        let mut converted_config: HashMap<String, String> = HashMap::new();

        match loaded_textures_config {
            Object(config) => {
                for entry in config.clone() {
                    match entry.1 {
                        Value::String(path) => {
                            converted_config.insert(entry.0, path);
                        }
                        _ => panic!("Expected texture path for texture ID {}", entry.0)
                    }
                }
            }
            _ => panic!("Invalid JSON structure. Expected 'textures' key-value object.")
        }

        TextureBag {
            config_data: converted_config,
            textures: HashMap::new(),
        }
    }

    /// Method tries to get texture from existing map.
    /// If texture was not found in map, method will try to get it's path from config,
    /// load texture and put it in map. Texture will also be returned.
    /// Display variable won't be used if texture was initialized on load.
    /// # Panics
    ///
    /// Method will panic if texture file is missing.
    pub fn get_texture<F>(&mut self, texture_id: String, facade: &F)
                          -> &glium::texture::Texture2d where F: Facade
    {
        if self.textures.get(&texture_id).is_none() {
            let texture_path = match self.config_data.get(&texture_id) {
                Some(path) => path.clone(),
                None => panic!("Unknown texture_id provided to bag: {}", &texture_id),
            };
            let loaded_texture = glium::texture::Texture2d::new(facade, Texture::from_file(texture_path.as_str()).as_raw_image_2d()).unwrap();
            self.textures.insert(
                texture_id.clone(),
                loaded_texture
            );
        }

        return self.textures.get(&texture_id).unwrap();
    }

    /// Removes texture from memory. Texture will be reloaded on next get_texture call.
    pub fn forget(&mut self, texture_id: String) {
        self.textures.remove(texture_id.as_str());
    }
}