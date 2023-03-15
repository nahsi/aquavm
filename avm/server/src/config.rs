/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::AVMDataStore;
use std::path::PathBuf;

/// Describes behaviour of the AVM.
pub struct AVMConfig<E> {
    /// Path to a AIR interpreter Wasm file.
    pub air_wasm_path: PathBuf,

    /// Maximum heap size in bytes available for the interpreter.
    pub max_heap_size: Option<u64>,

    /// Mask used to filter logs, for details see `log_utf8_string` in fluence-faas.
    pub logging_mask: i32,

    pub data_store: AVMDataStore<E>,
}
