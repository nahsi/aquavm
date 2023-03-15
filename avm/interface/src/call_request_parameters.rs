/*
 * Copyright 2021 Fluence Labs Limited
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

use super::JValue;
use crate::CallSeDeErrors;

use polyplets::SecurityTetraplet;
use serde::Deserialize;
use serde::Serialize;

use std::collections::HashMap;

pub type CallRequests = HashMap<u32, CallRequestParams>;

/// Contains arguments of a call instruction and all other necessary information
/// required for calling a service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallRequestParams {
    /// Id of a service that should be called.
    pub service_id: String,

    /// Name of a function from service identified by service_id that should be called.
    pub function_name: String,

    /// Arguments that should be passed to the function.
    pub arguments: Vec<JValue>,

    /// Tetraplets that should be passed to the service.
    pub tetraplets: Vec<Vec<SecurityTetraplet>>,
}

impl CallRequestParams {
    pub fn new(
        service_id: impl Into<String>,
        function_name: impl Into<String>,
        arguments: Vec<JValue>,
        tetraplets: Vec<Vec<SecurityTetraplet>>,
    ) -> Self {
        Self {
            service_id: service_id.into(),
            function_name: function_name.into(),
            arguments,
            tetraplets,
        }
    }

    pub(crate) fn from_raw(
        call_params: air_interpreter_interface::CallRequestParams,
    ) -> Result<Self, CallSeDeErrors> {
        let arguments: Vec<JValue> =
            serde_json::from_str(&call_params.arguments).map_err(|de_error| {
                CallSeDeErrors::CallParamsArgsDeFailed {
                    call_params: call_params.clone(),
                    de_error,
                }
            })?;

        let tetraplets: Vec<Vec<SecurityTetraplet>> = serde_json::from_str(&call_params.tetraplets)
            .map_err(|de_error| CallSeDeErrors::CallParamsTetrapletsDeFailed {
                call_params: call_params.clone(),
                de_error,
            })?;

        let call_params = Self {
            service_id: call_params.service_id,
            function_name: call_params.function_name,
            arguments,
            tetraplets,
        };

        Ok(call_params)
    }
}

pub(crate) fn from_raw_call_requests(
    raw_call_params: Vec<u8>,
) -> Result<CallRequests, CallSeDeErrors> {
    let call_requests: air_interpreter_interface::CallRequests =
        match serde_json::from_slice(&raw_call_params) {
            Ok(requests) => requests,
            Err(error) => {
                return Err(CallSeDeErrors::CallRequestsDeError {
                    raw_call_request: raw_call_params,
                    error,
                })
                .map_err(Into::into)
            }
        };

    call_requests
        .into_iter()
        .map(|(call_id, call_params)| -> Result<_, _> {
            let call_params = CallRequestParams::from_raw(call_params)?;
            Ok((call_id, call_params))
        })
        .collect::<Result<_, _>>()
}
