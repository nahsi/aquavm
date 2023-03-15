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

use crate::execution_step::execution_context::ExecutionCtx;
use crate::execution_step::instructions::ExecutionResult;
use crate::execution_step::resolver::prepare_last_error;
use crate::execution_step::resolver::resolve_ast_variable;
use crate::execution_step::resolver::resolve_ast_variable_wl;
use crate::JValue;

use air_parser::ast;

use std::borrow::Cow;

#[tracing::instrument(skip_all)]
pub(crate) fn are_matchable_eq<'ctx>(
    left: &ast::ImmutableValue<'_>,
    right: &ast::ImmutableValue<'_>,
    exec_ctx: &'ctx ExecutionCtx<'_>,
) -> ExecutionResult<bool> {
    use ast::ImmutableValue::*;

    match (left, right) {
        // TODO: introduce traits to refactor this in boxed values
        (InitPeerId, InitPeerId) => Ok(true),
        (InitPeerId, matchable) | (matchable, InitPeerId) => compare_matchable(
            matchable,
            exec_ctx,
            make_string_comparator(exec_ctx.run_parameters.init_peer_id.as_str()),
        ),

        (LastError(error_accessor), matchable) | (matchable, LastError(error_accessor)) => {
            let (value, _) = prepare_last_error(error_accessor, exec_ctx)?;
            compare_matchable(matchable, exec_ctx, make_object_comparator(value))
        }

        (Literal(left_name), Literal(right_name)) => Ok(left_name == right_name),
        (Literal(value), matchable) | (matchable, Literal(value)) => {
            compare_matchable(matchable, exec_ctx, make_string_comparator(value))
        }

        (Timestamp, Timestamp) => Ok(true),
        (Timestamp, matchable) | (matchable, Timestamp) => compare_matchable(
            matchable,
            exec_ctx,
            make_object_comparator(JValue::Number(exec_ctx.run_parameters.timestamp.into())),
        ),

        (TTL, TTL) => Ok(true),
        (TTL, matchable) | (matchable, TTL) => compare_matchable(
            matchable,
            exec_ctx,
            make_object_comparator(JValue::Number(exec_ctx.run_parameters.ttl.into())),
        ),

        (EmptyArray, EmptyArray) => Ok(true),
        (EmptyArray, matchable) | (matchable, EmptyArray) => {
            compare_matchable(matchable, exec_ctx, make_object_comparator(JValue::Array(vec![])))
        }

        (Boolean(left_boolean), Boolean(right_boolean)) => Ok(left_boolean == right_boolean),
        (Boolean(value), matchable) | (matchable, Boolean(value)) => {
            compare_matchable(matchable, exec_ctx, make_object_comparator((*value).into()))
        }

        (Number(left_number), Number(right_number)) => Ok(left_number == right_number),
        (Number(value), matchable) | (matchable, Number(value)) => {
            compare_matchable(matchable, exec_ctx, make_object_comparator(value.into()))
        }

        (Variable(left_variable), Variable(right_variable)) => {
            let (left_value, _) = resolve_ast_variable(left_variable, exec_ctx)?;
            let (right_value, _) = resolve_ast_variable(right_variable, exec_ctx)?;

            Ok(left_value == right_value)
        }

        (Variable(left_variable), VariableWithLambda(right_variable)) => {
            let (left_value, _) = resolve_ast_variable(left_variable, exec_ctx)?;
            let (right_value, _) = resolve_ast_variable_wl(right_variable, exec_ctx)?;

            Ok(left_value == right_value)
        }

        (VariableWithLambda(left_variable), Variable(right_variable)) => {
            let (left_value, _) = resolve_ast_variable_wl(left_variable, exec_ctx)?;
            let (right_value, _) = resolve_ast_variable(right_variable, exec_ctx)?;

            Ok(left_value == right_value)
        }

        (VariableWithLambda(left_variable), VariableWithLambda(right_variable)) => {
            let (left_value, _) = resolve_ast_variable_wl(left_variable, exec_ctx)?;
            let (right_value, _) = resolve_ast_variable_wl(right_variable, exec_ctx)?;

            Ok(left_value == right_value)
        }
    }
}

type Comparator<'a> = Box<dyn Fn(Cow<'_, JValue>) -> bool + 'a>;

fn compare_matchable<'ctx>(
    matchable: &ast::ImmutableValue<'_>,
    exec_ctx: &'ctx ExecutionCtx<'_>,
    comparator: Comparator<'ctx>,
) -> ExecutionResult<bool> {
    use ast::ImmutableValue::*;

    match matchable {
        InitPeerId => {
            let init_peer_id = exec_ctx.run_parameters.init_peer_id.clone();
            let jvalue = init_peer_id.as_str().into();
            Ok(comparator(Cow::Owned(jvalue)))
        }
        LastError(error_accessor) => {
            let (jvalue, _) = prepare_last_error(error_accessor, exec_ctx)?;
            Ok(comparator(Cow::Owned(jvalue)))
        }
        Literal(str) => {
            let jvalue = str.to_string().into();
            Ok(comparator(Cow::Owned(jvalue)))
        }
        Timestamp => {
            let jvalue = exec_ctx.run_parameters.timestamp.into();
            Ok(comparator(Cow::Owned(jvalue)))
        }
        TTL => {
            let jvalue = exec_ctx.run_parameters.ttl.into();
            Ok(comparator(Cow::Owned(jvalue)))
        }
        Number(number) => {
            let jvalue = number.into();
            Ok(comparator(Cow::Owned(jvalue)))
        }
        Boolean(bool) => {
            let jvalue = (*bool).into();
            Ok(comparator(Cow::Owned(jvalue)))
        }
        EmptyArray => {
            let jvalue = JValue::Array(vec![]);
            Ok(comparator(Cow::Owned(jvalue)))
        }
        Variable(variable) => {
            let (jvalue, _) = resolve_ast_variable(variable, exec_ctx)?;
            Ok(comparator(Cow::Owned(jvalue)))
        }
        VariableWithLambda(variable) => {
            let (jvalue, _) = resolve_ast_variable_wl(variable, exec_ctx)?;
            Ok(comparator(Cow::Owned(jvalue)))
        }
    }
}

fn make_string_comparator(comparable_string: &str) -> Comparator<'_> {
    use std::ops::Deref;

    Box::new(move |jvalue: Cow<'_, JValue>| -> bool {
        match jvalue.deref() {
            JValue::String(value) => value == comparable_string,
            _ => false,
        }
    })
}

fn make_object_comparator(comparable_value: JValue) -> Comparator<'static> {
    use std::ops::Deref;

    Box::new(move |jvalue: Cow<'_, JValue>| -> bool { jvalue.deref() == &comparable_value })
}
