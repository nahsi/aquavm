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

#[cfg(feature = "marine")]
use fluence_it_types::ne_vec::NEVec;
#[cfg(feature = "marine")]
use fluence_it_types::IValue;
#[cfg(feature = "marine")]
use marine_rs_sdk::marine;
use serde::Deserialize;
use serde::Serialize;

/// Parameters that a host side should pass to an interpreter and that necessary for execution.
#[cfg_attr(feature = "marine", marine)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RunParameters {
    /// Peer id of a peer that start this particle.
    pub init_peer_id: String,

    /// Peer id of a current peer.
    pub current_peer_id: String,

    /// Unix timestamp from a particle in milliseconds.
    /// It represents time when this particle was sent from the init peer id.
    pub timestamp: u64,

    /// TTL set by init peer id in milliseconds.
    pub ttl: u32,
}

impl RunParameters {
    pub fn new(init_peer_id: String, current_peer_id: String, timestamp: u64, ttl: u32) -> Self {
        Self {
            init_peer_id,
            current_peer_id,
            timestamp,
            ttl,
        }
    }

    #[cfg(feature = "marine")]
    pub fn into_ivalue(self) -> IValue {
        let run_parameters = vec![
            IValue::String(self.init_peer_id),
            IValue::String(self.current_peer_id),
            IValue::U64(self.timestamp),
            IValue::U32(self.ttl),
        ];
        // unwrap is safe here because run_parameters is non-empty array
        let run_parameters = NEVec::new(run_parameters).unwrap();
        IValue::Record(run_parameters)
    }
}
