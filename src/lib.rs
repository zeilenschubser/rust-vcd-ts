// (C) 2025 Zeilenschubser
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Cursor};
use tsify::{Tsify, declare};
use vcd::ScopeItem;
use wasm_bindgen::prelude::*;
use wasm_bindgen_console_logger::DEFAULT_LOGGER;

mod errors;
use errors::VCDFileError;

mod file_provider;
use file_provider::FileContentProvider;

// type defs
#[declare]
type VariableMapType = HashMap<String, String>;
#[declare]
type CycleToValueMap = HashMap<u64, String>;
#[declare]
type ValueMapType = HashMap<String, CycleToValueMap>;

#[wasm_bindgen(start)]
pub fn main() {
  log::set_logger(&DEFAULT_LOGGER).unwrap();
  log::set_max_level(log::LevelFilter::Debug);
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct VCDFile {
  filename: String,
  variable_map: VariableMapType,
  value_map: ValueMapType,
}

fn variable_map_from_scope_item_list(
  scope_stack: &mut Vec<String>,
  scope_items: Vec<ScopeItem>,
) -> Option<VariableMapType> {
  let mut variable_map = HashMap::new();
  for scope_item in scope_items {
    match scope_item {
      ScopeItem::Var(var) => {
        let full_var_name_list: Vec<String> = scope_stack
          .iter()
          .cloned()
          .chain(std::iter::once(var.reference))
          .collect();
        let full_var_name: String = full_var_name_list.join(".");
        variable_map.insert(full_var_name, var.code.to_string());
      }
      ScopeItem::Comment(comment) => {
        info!("found comment {}", comment);
      }
      ScopeItem::Scope(scope) => {
        if scope.scope_type == vcd::ScopeType::Module {
          scope_stack.push(scope.identifier);
        }
        let map = variable_map_from_scope_item_list(scope_stack, scope.items);
        scope_stack.pop();
        if let Some(map) = map {
          variable_map.extend(map);
        }
      }
      _ => {
        error!("TODO: implement scopeItem that is unhandled");
      }
    }
  }
  Some(variable_map)
}

fn value_map_from_parser<R>(parser: &mut vcd::Parser<R>) -> Result<ValueMapType, VCDFileError>
where
  R: BufRead,
{
  let mut map = ValueMapType::new();
  let mut time = 0;
  while let Some(parse_command) = parser.next() {
    if let Ok(command) = parse_command {
      match command {
        vcd::Command::Timestamp(t) => {
          time = t;
        }
        vcd::Command::ChangeScalar(id, value) => {
          map
            .entry(id.to_string())
            .or_insert_with(HashMap::new)
            .insert(time, value.to_string());
        }
        vcd::Command::ChangeVector(id, value) => {
          map
            .entry(id.to_string())
            .or_insert_with(HashMap::new)
            .insert(
              time,
              value
                .iter()
                .map(|slf: vcd::Value| slf.to_string())
                .collect(),
            );
        }
        vcd::Command::ChangeReal(id, value) => {
          map
            .entry(id.to_string())
            .or_insert_with(HashMap::new)
            .insert(time, format!("{:6.6}", value));
        }
        vcd::Command::ChangeString(id, value) => {
          map
            .entry(id.to_string())
            .or_insert_with(HashMap::new)
            .insert(time, value);
        }
        vcd::Command::Comment(_commentstr) => {
          // dont do anything
        }
        vcd::Command::Begin(_section) => {}
        vcd::Command::End(_section) => {}
        command => {
          error!("Unexpected {command:?} at line {line}", line = 0);
          println!("Unexpected {command:?} at line {line}", line = 0)
        }
      }
    } else if let Err(_err) = parse_command {
      // ?
      return Err(VCDFileError::ParsingFailCommandInvalid);
    }
  }
  Ok(map)
}

#[wasm_bindgen]
pub fn load_vcd_by_filename(filename: String) -> Result<VCDFile, VCDFileError> {
  let provider: FileContentProvider = FileContentProvider::new();
  match provider.get_content(filename.clone()) {
    Ok(content) => load_vcd(filename, content),
    Err(err) => Err(err),
  }
}

#[wasm_bindgen]
pub fn load_vcd(filename: String, content: String) -> Result<VCDFile, VCDFileError> {
  let mut parser = vcd::Parser::new(BufReader::new(Cursor::new(content)));
  // Parse the header and find the wires
  if let Ok(header) = parser.parse_header() {
    if let Some(variable_map) = variable_map_from_scope_item_list(&mut Vec::new(), header.items) {
      let values = value_map_from_parser(&mut parser);
      if let Ok(value_map) = values {
        return Ok(VCDFile {
          filename: filename,
          variable_map: variable_map,
          value_map: value_map,
        });
      } else if let Err(value_err) = values {
        return Err(value_err);
      } else {
        return Err(VCDFileError::ExtractionValueMapFail);
      }
    } else {
      return Err(VCDFileError::ExtractionVariableMapFail);
    }
  } else {
    return Err(VCDFileError::ParsingFailHeaderInvalid);
  }
}

#[cfg(test)]
mod tests {
  use crate::load_vcd_by_filename;
  #[test]
  fn test_load_vcd() {
    let filename = "../test.vcd";
    match load_vcd_by_filename(String::from(filename)) {
      Ok(vcd) => {
        for (key, value) in vcd.variable_map {
          println!(
            "- var: {} {} {}",
            !key.is_empty() && !value.is_empty(),
            key,
            value
          );
        }
      }
      Err(err) => {
        println!("ERROR: {:?}", err);
      }
    }
    assert!(false);
  }
}
