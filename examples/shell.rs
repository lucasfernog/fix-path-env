// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use fix_path_env::fix_all_vars;

fn main() {
  if let Err(e) = fix_all_vars() {
    println!("{}", e);
  }
}
