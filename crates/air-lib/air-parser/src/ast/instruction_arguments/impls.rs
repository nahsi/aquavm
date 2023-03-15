/*
 * Copyright 2022 Fluence Labs Limited
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

use super::ApResult;
use super::CallOutputValue;
use super::NewArgument;
use super::Scalar;
use super::Stream;
use crate::parser::lexer::AirPos;

impl<'i> NewArgument<'i> {
    pub fn name(&self) -> &'i str {
        match self {
            Self::Scalar(scalar) => scalar.name,
            Self::Stream(stream) => stream.name,
            Self::CanonStream(canon_stream) => canon_stream.name,
        }
    }
}

impl<'i> ApResult<'i> {
    pub fn scalar(name: &'i str, position: AirPos) -> Self {
        Self::Scalar(Scalar { name, position })
    }

    pub fn stream(name: &'i str, position: AirPos) -> Self {
        Self::Stream(Stream { name, position })
    }

    pub fn name(&self) -> &'i str {
        match self {
            Self::Scalar(scalar) => scalar.name,
            Self::Stream(stream) => stream.name,
        }
    }
}

impl<'i> CallOutputValue<'i> {
    pub fn scalar(name: &'i str, position: AirPos) -> Self {
        Self::Scalar(Scalar { name, position })
    }

    pub fn stream(name: &'i str, position: AirPos) -> Self {
        Self::Stream(Stream { name, position })
    }
}
