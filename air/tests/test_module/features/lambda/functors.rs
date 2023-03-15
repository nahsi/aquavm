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

use air::CatchableError;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

use std::cell::RefCell;

#[test]
fn length_functor_for_array_scalar() {
    let script = r#"
        (seq
            (call %init_peer_id% ("" "") [] variable) ; ok = [1,1,1]
            (call %init_peer_id% ("" "") [variable.length]) ; behaviour = echo
        )
        "#;

    let init_peer_id = "init_peer_id";
    let executor = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), script)
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_id).unwrap();
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::scalar(json!([1, 1, 1])),
        executed_state::scalar_number(3),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn length_functor_for_non_array_scalar() {
    let result_jvalue = "string_jvalue";
    let script = f!(r#"
        (seq
            (call %init_peer_id% ("" "") [] variable) ; ok = "{result_jvalue}"
            (call %init_peer_id% ("" "") [variable.length]) ; behaviour = echo
        )
        "#);

    let init_peer_id = "init_peer_id";
    let executor = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), &script)
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_id).unwrap();
    check_error(
        &result,
        CatchableError::LengthFunctorAppliedToNotArray(json!(result_jvalue)),
    );
}

#[test]
fn length_functor_for_stream() {
    let script = r#"
        (seq
            (seq
                (ap 1 $stream)
                (ap 1 $stream))
            (seq
                (canon %init_peer_id% $stream #stream)
                (call %init_peer_id% ("" "") [#stream.length]) ; behaviour = echo
            )
        )
        "#;

    let init_peer_id = "init_peer_id";
    let executor = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), script)
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_id).unwrap();
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::canon(json!({
            "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "init_peer_id", "service_id": ""},
            "values": [
                {
                    "result": 1,
                    "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "init_peer_id", "service_id": ""},
                    "trace_pos": 0,
                },
                {
                    "result": 1,
                    "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "init_peer_id", "service_id": ""},
                    "trace_pos": 1,
                },
            ]
        })),
        executed_state::scalar_number(2),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn length_functor_for_empty_stream() {
    let script = r#"
        (new $stream
            (seq
                (canon %init_peer_id% $stream #canon_stream)
                (call %init_peer_id% ("" "") [#canon_stream.length]) ; behaviour = echo
            )
        )
        "#;

    let init_peer_id = "init_peer_id";
    let executor = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), script)
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_id).unwrap();
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "init_peer_id", "service_id": ""},
                "values": []} ),
        ),
        executed_state::scalar_number(0),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn length_functor_for_canon_stream() {
    let script = r#"
        (seq
            (seq
                (ap 1 $stream)
                (ap 1 $stream))
            (seq
                (canon %init_peer_id% $stream #canon_stream)
                (call %init_peer_id% ("" "") [#canon_stream.length]) ; behaviour = echo
            )
        )
        "#;

    let init_peer_id = "init_peer_id";
    let executor = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), script)
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_id).unwrap();
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "init_peer_id", "service_id": ""},
                "values": [{"result": 1, "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "init_peer_id", "service_id": ""}, "trace_pos": 0},
                           {"result": 1, "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "init_peer_id", "service_id": ""}, "trace_pos": 1}
                ]} ),
        ),
        executed_state::scalar_number(2),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn length_functor_for_empty_canon_stream() {
    let script = r#"
        (new $stream
            (seq
                (canon %init_peer_id% $stream #canon_stream)
                (call %init_peer_id% ("" "") [#canon_stream.length]) ; behaviour = echo
            )
        )
        "#;

    let init_peer_id = "init_peer_id";
    let executor = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), script)
        .expect("invalid test AIR script");

    let result = executor.execute_one(init_peer_id).unwrap();
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "init_peer_id", "service_id": ""}, "values": []} ),
        ),
        executed_state::scalar_number(0),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn functor_dont_influence_tetraplet() {
    let set_variable_peer_id = "set_variable_peer_id";
    let set_variable_peer_result = json!({"field": [1,2,3]});
    let mut set_variable_vm = create_avm(
        set_variable_call_service(set_variable_peer_result.clone()),
        set_variable_peer_id,
    );

    let tetraplet_catcher_peer_id = "tetraplet_catcher_peer_id";
    let (call_service, actual_tetraplet) = tetraplet_host_function(echo_call_service());
    let mut tetraplet_catcher_vm = create_avm(call_service, tetraplet_catcher_peer_id);

    let script = f!(r#"
        (seq
            (call "{set_variable_peer_id}" ("" "") [] scalar)
            (seq
                (ap scalar.$.field field)
                (seq
                    (ap field.length length)
                    (call "{tetraplet_catcher_peer_id}" ("" "") [length])
                )
            )
        )
        "#);

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(tetraplet_catcher_vm, <_>::default(), &script, "", result.data);
    let actual_trace = trace_from_result(&result);

    let expected_tetraplet = RefCell::new(vec![vec![SecurityTetraplet::new("", "", "", ".length")]]);
    assert_eq!(actual_tetraplet.as_ref(), &expected_tetraplet);

    let expected_trace = vec![
        executed_state::scalar(set_variable_peer_result),
        executed_state::scalar_number(3),
    ];
    assert_eq!(actual_trace, expected_trace);
}
