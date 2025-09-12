// (C) 2025 Zeilenschubser
use crate::VCDFileError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern "C" {
  fn readFileSync(filename: String, encoding: String) -> String;
}

pub struct FileContentProvider {}

#[cfg(target_arch = "wasm32")]
impl FileContentProvider {
  pub fn new() -> Self {
    FileContentProvider {}
  }
  pub fn get_content(&self, path: String) -> Result<String, VCDFileError> {
    Ok(readFileSync(path, "utf8".to_string()))
  }
}

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::BufReader;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;

#[cfg(not(target_arch = "wasm32"))]
impl FileContentProvider {
  pub fn new() -> Self {
    FileContentProvider {}
  }
  pub fn get_content(&self, path: String) -> Result<String, VCDFileError> {
    if let Ok(file) = File::open(path) {
      let mut reader = BufReader::new(file);
      let mut str = String::new();
      if let Ok(_result) = reader.read_to_string(&mut str) {
        return Ok(str);
      }
      return Err(VCDFileError::FileReadError);
    }
    return Err(VCDFileError::FileNotOpened);
  }
}
