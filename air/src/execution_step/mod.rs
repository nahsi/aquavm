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

mod boxed_value;
mod errors;
pub(crate) mod execution_context;
mod instructions;
mod lambda_applier;
mod resolver;

const PEEK_ALLOWED_ON_NON_EMPTY: &str = "peek always return elements inside fold,\
            this guaranteed by implementation of next and avoiding empty folds";

const TETRAPLET_IDX_CORRECT: &str = "selects always return a correct index inside stream";

pub use errors::CatchableError;
pub use errors::ExecutionError;
pub use errors::UncatchableError;
pub use execution_context::LastErrorObjectError;
pub use lambda_applier::LambdaError;

pub mod errors_prelude {
    pub use super::CatchableError;
    pub use super::ExecutionError;
    pub use super::UncatchableError;
}

pub(super) use self::instructions::ExecutableInstruction;
pub(super) use self::instructions::FoldState;
pub(super) use boxed_value::Generation;
pub(super) use boxed_value::ScalarRef;
pub(super) use boxed_value::Stream;
pub(super) use boxed_value::ValueAggregate;
pub(crate) use errors::Joinable;
pub(crate) use errors::LastErrorAffectable;
pub(crate) use execution_context::ExecutionCtx;
pub(crate) use execution_context::LastError;

pub(crate) use air_trace_handler::TraceHandler;

use std::rc::Rc;

type ExecutionResult<T> = std::result::Result<T, ExecutionError>;
type RcSecurityTetraplet = Rc<crate::SecurityTetraplet>;
type RcSecurityTetraplets = Vec<RcSecurityTetraplet>;
