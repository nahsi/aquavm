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

/// Print out each instruction name at the beginning of its execution_step.
pub const INSTRUCTION: &str = "instruction";

/// Print out data cache at the beginning of each instruction execution_step.
pub const DATA_CACHE: &str = "data_cache";

/// Print out next_peer_pks at the beginning of each instruction execution_step.
pub const NEXT_PEER_PKS: &str = "next_peer_pks";

/// Print out subgraph_complete value at the beginning of each instruction execution_step.
pub const SUBGRAPH_COMPLETE: &str = "subgraph_complete";

/// Print out count of element in the current subgraph at the beginning of each instruction execution_step.
pub const SUBGRAPH_ELEMENTS: &str = "subgraph_elements_count";

/// Print out state of data cache at the beginning of each instruction execution_step.
pub const NEW_EXECUTED_TRACE: &str = "new_executed_trace";

/// Print out logs at the executed states merging stage.
pub const EXECUTED_TRACE_MERGE: &str = "executed_trace_merge";

/// Print out running arguments and params of a script.
pub const RUN_PARAMS: &str = "initial_params";

/// Print out state of data cache at the beginning of each instruction execution_step.
pub const EXECUTED_STATE_CHANGING: &str = "executed_state_changing";

/// Print log if call is postponed due the join behaviour.
pub const JOIN_BEHAVIOUR: &str = "join_behaviour";

/// This map should be used by rust-sdk logger that allows print only necessary targets by id.
pub const TARGET_MAP: [(&str, i32); 10] = [
    (INSTRUCTION, 1 << 1),
    (DATA_CACHE, 1 << 2),
    (NEXT_PEER_PKS, 1 << 3),
    (SUBGRAPH_COMPLETE, 1 << 4),
    (SUBGRAPH_ELEMENTS, 1 << 5),
    (NEW_EXECUTED_TRACE, 1 << 6),
    (EXECUTED_TRACE_MERGE, 1 << 7),
    (RUN_PARAMS, 1 << 8),
    (EXECUTED_STATE_CHANGING, 1 << 9),
    (JOIN_BEHAVIOUR, 1 << 10),
];
