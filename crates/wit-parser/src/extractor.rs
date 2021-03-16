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

mod functions;
mod wit;

pub use functions::*;
pub use wit::*;

use crate::Result;
use crate::WITParserError;
use std::path::Path;

pub fn module_interface<P>(module_path: P) -> Result<ServiceInterface>
where
    P: AsRef<Path>,
{
    use fce_wit_interfaces::FCEWITInterfaces;

    let module = walrus::ModuleConfig::new()
        .parse_file(module_path)
        .map_err(WITParserError::CorruptedWasmFile)?;

    let raw_custom_section = extract_custom_section(&module)?;
    let custom_section_bytes = raw_custom_section.as_ref();
    let wit = extract_wit_from_bytes(custom_section_bytes)?;
    let fce_interface = FCEWITInterfaces::new(wit);

    get_interface(&fce_interface)
}
