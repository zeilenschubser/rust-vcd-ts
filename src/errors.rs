// (C) 2025 Zeilenschubser

use crate::JsValue;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Tsify, Serialize, Deserialize)]
pub enum VCDFileError {
  FileNotOpened,
  FileReadError,
  WasmWindowNotFound,
  ParsingFailHeaderInvalid,
  ParsingFailCommandInvalid,
  ExtractionValueMapFail,
  ExtractionVariableMapFail,
}

// #[wasm_bindgen]
impl VCDFileError {
  pub fn to_str(&self) -> &'static str {
    match self {
      VCDFileError::FileNotOpened => "FileNotOpened",
      VCDFileError::FileReadError => "FileReadError",
      VCDFileError::WasmWindowNotFound => "WasmWindowNotFound",
      VCDFileError::ParsingFailHeaderInvalid => "ParsingFailHeaderInvalid",
      VCDFileError::ParsingFailCommandInvalid => "ParsingFailCommandInvalid",
      VCDFileError::ExtractionValueMapFail => "ExtractionValueMapFail",
      VCDFileError::ExtractionVariableMapFail => "ExtractionVariableMapFail",
    }
  }
}
impl std::error::Error for VCDFileError {}

impl From<VCDFileError> for JsValue {
  fn from(err: VCDFileError) -> Self {
    JsValue::from(err.to_str())
  }
}

impl std::fmt::Display for VCDFileError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_str())
  }
}
