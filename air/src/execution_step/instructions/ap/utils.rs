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

use super::ExecutionResult;
use crate::execution_step::execution_context::StreamValueDescriptor;
use crate::execution_step::Generation;
use crate::execution_step::ValueAggregate;

use air_parser::ast;
use air_trace_handler::merger::MergerApResult;

pub(super) fn generate_value_descriptor<'stream>(
    value: ValueAggregate,
    stream: &'stream ast::Stream<'_>,
    ap_result: &MergerApResult,
) -> StreamValueDescriptor<'stream> {
    use air_trace_handler::merger::ValueSource;

    match ap_result {
        MergerApResult::NotMet => StreamValueDescriptor::new(
            value,
            stream.name,
            ValueSource::PreviousData,
            Generation::Last,
            stream.position,
        ),
        MergerApResult::Met(met_result) => StreamValueDescriptor::new(
            value,
            stream.name,
            met_result.value_source,
            Generation::Nth(met_result.generation),
            stream.position,
        ),
    }
}

pub(super) fn try_match_trace_to_instr(merger_ap_result: &MergerApResult, instr: &ast::Ap<'_>) -> ExecutionResult<()> {
    use crate::execution_step::UncatchableError::ApResultNotCorrespondToInstr;
    use ast::ApResult;

    match (&instr.result, merger_ap_result) {
        (ApResult::Stream(_), MergerApResult::Met(_)) => Ok(()),
        (_, MergerApResult::NotMet) => Ok(()),
        _ => Err(ApResultNotCorrespondToInstr(merger_ap_result.clone()).into()),
    }
}
