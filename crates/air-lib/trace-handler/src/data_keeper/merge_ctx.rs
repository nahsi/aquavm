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

use super::ExecutionTrace;
use super::KeeperError;
use super::KeeperResult;
use super::TraceSlider;
use crate::TracePos;

/// Contains all necessary information about data.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct MergeCtx {
    pub slider: TraceSlider,
}

impl MergeCtx {
    pub(crate) fn from_trace(trace: ExecutionTrace) -> Self {
        let slider = TraceSlider::new(trace);

        Self { slider }
    }

    pub(crate) fn try_get_generation(&self, position: TracePos) -> KeeperResult<u32> {
        use air_interpreter_data::*;

        let state = self
            .slider
            .state_at_position(position)
            .ok_or_else(|| KeeperError::NoElementAtPosition {
                position,
                trace_len: self.slider.trace_len(),
            })?;

        match state {
            ExecutedState::Call(CallResult::Executed(ValueRef::Stream { generation, .. })) => Ok(*generation),
            // such Aps are always preceded by Fold where corresponding stream could be used,
            // so it's been already checked that res_generation is well-formed
            // and accessing 0th element is safe here
            ExecutedState::Ap(ap_result) => Ok(ap_result.res_generations[0]),
            state => Err(KeeperError::NoStreamState { state: state.clone() }),
        }
    }
}
