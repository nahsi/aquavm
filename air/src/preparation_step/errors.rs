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

use crate::ToErrorCode;
use air_interpreter_data::data_version;
use air_interpreter_data::Versions;

use serde_json::Error as SerdeJsonError;
use strum::IntoEnumIterator;
use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
use thiserror::Error as ThisError;

/// Errors happened during the interpreter preparation step.
#[derive(Debug, EnumDiscriminants, ThisError)]
#[strum_discriminants(derive(EnumIter))]
pub enum PreparationError {
    /// Error occurred while parsing AIR script
    #[error("air can't be parsed:\n{0}")]
    AIRParseError(String),

    /// Errors occurred on executed trace deserialization.
    #[error(
        "an error occurred while data deserialization: {error:?}.\n\
        AquaVM version is {} and it expect data of {} version,\
        it's failed to get version of AquaVM produced this data.\n\
        data: {data:?}",
        super::interpreter_version(),
        data_version()
    )]
    DataDeFailed { data: Vec<u8>, error: SerdeJsonError },

    /// Errors occurred on executed trace deserialization
    /// when it was possible to recover versions.
    #[error(
        "an error occurred while data deserialization: {error:?}.\n\
        AquaVM's version is {} and it expects data of {} version.\n\
        Supplied data version is {}, it's produced by AquaVM of {} version.\n\
        Data: {data:?}",
        super::interpreter_version(),
        data_version(),
        versions.data_version,
        versions.interpreter_version,
    )]
    DataDeFailedWithVersions {
        data: Vec<u8>,
        error: SerdeJsonError,
        versions: Versions,
    },

    /// Error occurred on call results deserialization.
    #[error(
        "error occurred while deserialize call results: {error:?}.\n\
    Call results: {call_results:?}"
    )]
    CallResultsDeFailed {
        call_results: Vec<u8>,
        error: SerdeJsonError,
    },

    /// Error occurred when a version of interpreter produced supplied data is less then minimal.
    #[error("supplied data was produced by `{actual_version}` version of interpreter, but minimum `{required_version}` version is required")]
    UnsupportedInterpreterVersion {
        actual_version: semver::Version,
        required_version: semver::Version,
    },
}

impl ToErrorCode for PreparationError {
    fn to_error_code(&self) -> i64 {
        use crate::utils::PREPARATION_ERROR_START_ID;
        crate::generate_to_error_code!(self, PreparationError, PREPARATION_ERROR_START_ID)
    }
}

impl PreparationError {
    pub fn data_de_failed(data: Vec<u8>, error: SerdeJsonError) -> Self {
        Self::DataDeFailed { data, error }
    }

    pub fn data_de_failed_with_versions(data: Vec<u8>, error: SerdeJsonError, versions: Versions) -> Self {
        Self::DataDeFailedWithVersions { data, error, versions }
    }

    pub fn call_results_de_failed(call_results: Vec<u8>, error: SerdeJsonError) -> Self {
        Self::CallResultsDeFailed { call_results, error }
    }

    pub fn unsupported_interpreter_version(actual_version: semver::Version, required_version: semver::Version) -> Self {
        Self::UnsupportedInterpreterVersion {
            actual_version,
            required_version,
        }
    }
}
