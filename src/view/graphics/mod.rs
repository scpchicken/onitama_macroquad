use macroquad::prelude::*;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub async fn get_image_hash() -> HashMap<String, Texture2D> {
  let image_name_regex = Regex::new(r"assets\\(.+?)\.png").unwrap();
  let image_vec = fs::read_dir("assets").unwrap();

  let mut image_hash: HashMap<String, Texture2D> = HashMap::new();

  for image in image_vec {
    let image_string = format!("{}", image.unwrap().path().display());
    image_hash.insert(
      image_name_regex
        .captures(&image_string)
        .unwrap()
        .get(1)
        .map_or("".to_string(), |m| m.as_str().to_string()),
      load_texture(&image_string).await.unwrap(),
    );
  }

  image_hash
}
