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

mod impls;
mod traits;

use non_empty_vec::NonEmpty;
use serde::Deserialize;
use serde::Serialize;

// TODO: rename lambda to smth more appropriate
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum LambdaAST<'input> {
    /// Various functors that could applied to a value.
    Functor(Functor),
    /// Each value in AIR could be represented as a tree and
    /// this variant acts as a path in such trees.
    #[serde(borrow)]
    ValuePath(NonEmpty<ValueAccessor<'input>>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ValueAccessor<'input> {
    // (.)?[$idx]
    ArrayAccess { idx: u32 },

    // .field
    FieldAccessByName { field_name: &'input str },

    // (.)?[field]
    FieldAccessByScalar { scalar_name: &'input str },

    // needed to allow parser catch all errors from a lambda expression without stopping
    // on the very first one. Although, this variant is guaranteed not to be present in a lambda.
    Error,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Functor {
    /// Returns a length of a value if this value has array type (json array or canon stream)
    /// or a error if not.
    Length,
}
