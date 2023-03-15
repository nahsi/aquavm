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

use super::*;
use crate::execution_step::execution_context::*;
use crate::execution_step::Generation;
use crate::execution_step::ValueAggregate;
use crate::UncatchableError;

use air_interpreter_data::CallResult;
use air_interpreter_data::TracePos;
use air_interpreter_data::ValueRef;
use air_parser::ast::CallOutputValue;
use air_trace_handler::merger::ValueSource;
use air_trace_handler::TraceHandler;

pub(crate) fn populate_context_from_peer_service_result<'i>(
    executed_result: ValueAggregate,
    output: &CallOutputValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<CallResult> {
    let cid = exec_ctx
        .cid_state
        .value_tracker
        .record_value(executed_result.result.clone())
        .map_err(UncatchableError::from)?;

    match output {
        CallOutputValue::Scalar(scalar) => {
            exec_ctx.scalars.set_scalar_value(scalar.name, executed_result)?;
            Ok(CallResult::executed_scalar(cid))
        }
        CallOutputValue::Stream(stream) => {
            let value_descriptor = StreamValueDescriptor::new(
                executed_result,
                stream.name,
                ValueSource::PreviousData,
                Generation::Last,
                stream.position,
            );
            let generation = exec_ctx.streams.add_stream_value(value_descriptor)?;
            Ok(CallResult::executed_stream(cid, generation))
        }
        // by the internal conventions if call has no output value,
        // corresponding data should have scalar type
        CallOutputValue::None => Ok(CallResult::executed_scalar(cid)),
    }
}

pub(crate) fn populate_context_from_data<'i>(
    value: ValueRef,
    tetraplet: RcSecurityTetraplet,
    trace_pos: TracePos,
    value_source: ValueSource,
    output: &CallOutputValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<ValueRef> {
    match (output, value) {
        (CallOutputValue::Scalar(scalar), ValueRef::Scalar(cid)) => {
            let value = exec_ctx.cid_state.get_value_by_cid(&cid)?;
            let result = ValueAggregate::new(value, tetraplet, trace_pos);
            exec_ctx.scalars.set_scalar_value(scalar.name, result)?;
            Ok(ValueRef::Scalar(cid))
        }
        (CallOutputValue::Stream(stream), ValueRef::Stream { cid, generation }) => {
            let value = exec_ctx.cid_state.get_value_by_cid(&cid)?;
            let result = ValueAggregate::new(value, tetraplet, trace_pos);
            let value_descriptor = StreamValueDescriptor::new(
                result,
                stream.name,
                value_source,
                Generation::Nth(generation),
                stream.position,
            );
            let resulted_generation = exec_ctx.streams.add_stream_value(value_descriptor)?;

            let result = ValueRef::Stream {
                cid,
                generation: resulted_generation,
            };
            Ok(result)
        }
        // by the internal conventions if call has no output value,
        // corresponding data should have scalar type
        (CallOutputValue::None, value @ ValueRef::Scalar(_)) => Ok(value),
        (_, value) => Err(ExecutionError::Uncatchable(
            UncatchableError::CallResultNotCorrespondToInstr(value),
        )),
    }
}

/// Writes an executed state of a particle being sent to remote node.
pub(crate) fn handle_remote_call(peer_pk: String, exec_ctx: &mut ExecutionCtx<'_>, trace_ctx: &mut TraceHandler) {
    exec_ctx.next_peer_pks.push(peer_pk);
    exec_ctx.make_subgraph_incomplete();

    let new_call_result = CallResult::sent_peer_id(exec_ctx.run_parameters.current_peer_id.clone());
    trace_ctx.meet_call_end(new_call_result);
}
