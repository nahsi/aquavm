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

use air::CatchableError;
use air_test_utils::prelude::*;

#[test]
fn match_equal() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = f!(r#"
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") ["value_1"] value_1)
                    (call "{set_variable_peer_id}" ("" "") ["value_1"] value_2)
                )
                (xor
                    (match value_1 value_2
                        (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#);

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string("result_1");

    assert_eq!(actual_trace.len(), 3);
    assert_eq!(actual_trace[2.into()], expected_state);
}

#[test]
fn match_not_equal() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = f!(r#"
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") ["value_1"] value_1)
                    (call "{set_variable_peer_id}" ("" "") ["value_2"] value_2)
                )
                (xor
                    (match value_1 value_2
                        (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#);

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string("result_2");

    assert_eq!(actual_trace.len(), 3);
    assert_eq!(actual_trace[2.into()], expected_state);
}

#[test]
fn match_with_string() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = f!(r#"
            (seq
                (call "{set_variable_peer_id}" ("" "") ["value_1"] value_1)
                (xor
                    (match value_1 "value_1"
                        (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#);

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string("result_1");

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[1.into()], expected_state);
}

#[test]
fn match_with_init_peer_id() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = f!(r#"
            (seq
                (call "{set_variable_peer_id}" ("" "") ["{local_peer_id}"] value_1)
                (xor
                    (match value_1 %init_peer_id%
                        (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#);

    let test_params = TestRunParameters::from_init_peer_id(local_peer_id);
    let result = checked_call_vm!(set_variable_vm, test_params.clone(), &script, "", "");
    let result = checked_call_vm!(vm, test_params, script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_executed_call_result = executed_state::scalar_string("result_1");

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[1.into()], expected_executed_call_result);
}

#[test]
fn match_with_timestamp() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let timestamp = 1337;
    let script = f!(r#"
            (seq
                (call "{set_variable_peer_id}" ("" "") [{timestamp}] value_1)
                (xor
                    (match value_1 %timestamp%
                        (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#);

    let test_params = TestRunParameters::from_timestamp(timestamp);
    let result = checked_call_vm!(set_variable_vm, test_params.clone(), &script, "", "");
    let result = checked_call_vm!(vm, test_params, script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_executed_call_result = executed_state::scalar_string("result_1");

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[1.into()], expected_executed_call_result);
}

#[test]
fn match_with_ttl() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let ttl = 1337;
    let script = f!(r#"
            (seq
                (call "{set_variable_peer_id}" ("" "") [{ttl}] value_1)
                (xor
                    (match value_1 %ttl%
                        (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#);

    let test_params = TestRunParameters::from_ttl(ttl);
    let result = checked_call_vm!(set_variable_vm, test_params.clone(), &script, "", "");
    let result = checked_call_vm!(vm, test_params, script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_executed_call_result = executed_state::scalar_string("result_1");

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[1.into()], expected_executed_call_result);
}

#[test]
fn match_with_equal_numbers() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = "
            (xor
                (match 1 1
                    (null)
                )
                (null)
            )";

    let result = call_vm!(vm, <_>::default(), script, "", "");

    assert!(is_interpreter_succeded(&result));
}

#[test]
fn match_without_xor() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = f!(r#"
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") ["value_1"] value_1)
                    (call "{set_variable_peer_id}" ("" "") ["value_2"] value_2)
                )
                (match value_1 value_2
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                )
            )"#);

    let result = call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = call_vm!(vm, <_>::default(), &script, "", result.data);

    let expected_error = CatchableError::MatchValuesNotEqual;
    assert!(check_error(&result, expected_error));

    let result = call_vm!(vm, <_>::default(), script, "", result.data);

    let expected_error = CatchableError::MatchValuesNotEqual;
    assert!(check_error(&result, expected_error));
}

#[test]
fn match_with_two_xors() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(set_variable_call_service(serde_json::json!(false)), local_peer_id);

    let local_peer_id_2 = "local_peer_id_2";

    let script = f!(r#"
            (xor
                (seq
                    (seq
                        (call "{local_peer_id}" ("getDataSrv" "condition") [] condition)
                        (call "{local_peer_id}" ("getDataSrv" "relay") [] relay)
                    )
                    (xor
                        (match condition true
                            (call "{local_peer_id}" ("println" "print") ["it is true"])
                        )
                        (call "{local_peer_id_2}" ("println" "print") ["it is false"])
                    )
                )
                (call "{local_peer_id}" ("errorHandlingSrv" "error") [%last_error%])
            )
            "#);

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    let mut actual_trace = trace_from_result(&result);
    let expected_executed_call_result = executed_state::request_sent_by(local_peer_id);

    assert_eq!(actual_trace.pop().unwrap(), expected_executed_call_result);
}

// https://github.com/fluencelabs/aquavm/issues/165
#[test]
fn issue_165() {
    let result_setter_peer_id = "result_setter_peer_id";
    let mut result_setter = create_avm(
        set_variable_call_service(serde_json::json!({"success": true})),
        result_setter_peer_id,
    );

    let echo_peer_id = "echo_peer_id";
    let mut echo_peer = create_avm(echo_call_service(), echo_peer_id);

    let script = f!(r#"
        (seq
            (call "{result_setter_peer_id}" ("" "") ["set_result"] result)
            (seq
                (xor
                    (match result.$.success! true
                        (ap 1 $results)
                    )
                    (ap 2 $results)
                )
                (seq
                    (canon "{echo_peer_id}" $results #results)
                    (call "{echo_peer_id}" ("callbackSrv" "response") [#results.$.[0]!])
                )
            )
        )
    "#);

    let setter_result = checked_call_vm!(result_setter, <_>::default(), &script, "", "");
    let echo_result = checked_call_vm!(echo_peer, <_>::default(), &script, "", setter_result.data);

    let trace = trace_from_result(&echo_result);
    assert_eq!(trace.last().unwrap(), &executed_state::scalar(json!(1)));
}
