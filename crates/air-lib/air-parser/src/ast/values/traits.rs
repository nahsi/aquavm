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
use std::fmt;

impl fmt::Display for Scalar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for ScalarWithLambda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name, self.lambda)
    }
}

impl fmt::Display for Stream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for CanonStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for CanonStreamWithLambda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name, self.lambda)
    }
}

impl fmt::Display for ImmutableVariable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ImmutableVariable::*;

        match self {
            Scalar(scalar) => write!(f, "{scalar}"),
            CanonStream(canon_stream) => write!(f, "{canon_stream}"),
        }
    }
}

impl fmt::Display for ImmutableVariableWithLambda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ImmutableVariableWithLambda::*;

        match self {
            Scalar(scalar) => write!(f, "{scalar}"),
            CanonStream(canon_stream) => write!(f, "{canon_stream}"),
        }
    }
}
