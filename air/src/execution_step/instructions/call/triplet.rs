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

use super::ExecutionCtx;
use super::ExecutionResult;
use crate::execution_step::CatchableError;
use crate::JValue;

use air_parser::ast;
use polyplets::ResolvedTriplet;

/// Resolve variables, literals, etc in the `Triplet`, and build a `ResolvedTriplet`.
pub(crate) fn resolve<'i>(triplet: &ast::Triplet<'i>, ctx: &ExecutionCtx<'i>) -> ExecutionResult<ResolvedTriplet> {
    let ast::Triplet {
        peer_id: peer_pk,
        service_id,
        function_name,
    } = triplet;

    let peer_pk = resolve_peer_id_to_string(peer_pk, ctx)?;
    let service_id = resolve_to_string(service_id, ctx)?;
    let function_name = resolve_to_string(function_name, ctx)?;

    Ok(ResolvedTriplet {
        peer_pk,
        service_id,
        function_name,
    })
}

/// Resolve peer id to string by either resolving variable from `ExecutionCtx`, taking literal value, or etc.
// TODO: return Rc<String> to avoid excess cloning
// TODO: move this function into resolve in boxed value PR
pub(crate) fn resolve_peer_id_to_string<'i>(
    value: &ast::ResolvableToPeerIdVariable<'i>,
    exec_ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<String> {
    use crate::execution_step::resolver;
    use ast::ResolvableToPeerIdVariable::*;

    let ((jvalue, _), name) = match value {
        InitPeerId => return Ok(exec_ctx.run_parameters.init_peer_id.to_string()),
        Literal(value) => return Ok(value.to_string()),
        Scalar(scalar) => (resolver::resolve_ast_scalar(scalar, exec_ctx)?, scalar.name),
        ScalarWithLambda(scalar) => (resolver::resolve_ast_scalar_wl(scalar, exec_ctx)?, scalar.name),
        CanonStreamWithLambda(canon_stream) => (
            resolver::resolve_ast_canon_wl(canon_stream, exec_ctx)?,
            canon_stream.name,
        ),
    };

    try_jvalue_to_string(jvalue, name)
}

/// Resolve value to string by either resolving variable from `ExecutionCtx`, taking literal value, or etc.
// TODO: return Rc<String> to avoid excess cloning
// TODO: move this function into resolve in boxed value PR
pub(crate) fn resolve_to_string<'i>(
    value: &ast::ResolvableToStringVariable<'i>,
    exec_ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<String> {
    use crate::execution_step::resolver;
    use ast::ResolvableToStringVariable::*;

    let ((jvalue, _), name) = match value {
        Literal(value) => return Ok(value.to_string()),
        Scalar(scalar) => (resolver::resolve_ast_scalar(scalar, exec_ctx)?, scalar.name),
        ScalarWithLambda(scalar) => (resolver::resolve_ast_scalar_wl(scalar, exec_ctx)?, scalar.name),
        CanonStreamWithLambda(canon_stream) => (
            resolver::resolve_ast_canon_wl(canon_stream, exec_ctx)?,
            canon_stream.name,
        ),
    };

    try_jvalue_to_string(jvalue, name)
}

fn try_jvalue_to_string(jvalue: JValue, variable_name: impl Into<String>) -> ExecutionResult<String> {
    match jvalue {
        JValue::String(s) => Ok(s),
        _ => Err(CatchableError::NonStringValueInTripletResolution {
            variable_name: variable_name.into(),
            actual_value: jvalue,
        }
        .into()),
    }
}
