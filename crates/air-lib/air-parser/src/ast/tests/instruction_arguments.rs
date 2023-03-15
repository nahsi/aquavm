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

use crate::ast::ImmutableValue;

use air_lambda_ast::LambdaAST;
use air_lambda_ast::ValueAccessor;

#[test]
// https://github.com/fluencelabs/aquavm/issues/263
fn issue_263() {
    let val = ImmutableValue::LastError(Some(
        LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
            field_name: "message",
        }])
        .unwrap(),
    ));
    assert_eq!(val.to_string(), "%last_error%.$.message");
}
