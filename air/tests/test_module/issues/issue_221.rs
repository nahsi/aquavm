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

use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

#[test]
// test for github.com/fluencelabs/aquavm/issues/221
fn issue_221() {
    let peer_1_id = "peer_1_id";
    let peer_2_id = "peer_2_id";
    let join_1_id = "join_1_id";
    let join_2_id = "join_2_id";
    let set_variable_id = "set_variable_id";

    let peer_1_value = "peer_1_value";
    let peer_2_value = "peer_2_value";

    let script = f!(r#"
        (seq
            (seq
                (seq
                    ;; let's peers be an array of two values [peer_1_id, peer_2_id]
                    (call "{set_variable_id}" ("" "") [] peers) ; ok = ["{peer_1_id}", "{peer_2_id}"]
                    (fold peers peer
                        (par
                            (seq
                                (call peer ("" "") [peer] value) ; map = {{"{peer_1_id}": "{peer_1_value}", "{peer_2_id}": "{peer_2_value}"}}
                                ;; it's crucial to reproduce this bug to add value to stream
                                ;; with help of ap instruction
                                (ap value $stream)
                            )
                            (next peer)
                        )
                    )
                )
                ;; join streams on join_1/join_2 peers in such a way that will have different state:
                ;; join_1 $stream: [peer_1_value, peer_2_value]
                ;; join_2 $stream: [peer_2_value, peer_1_value]
                (fold $stream iterator
                    ;; here it'll encounter a bug in trace handler, because fold won't shuffle lores in
                    ;; appropriate way and state for (1) is returned
                    (par
                        (par
                            (call "{join_1_id}" ("" "") [iterator]) ; behaviour = echo
                            (call "{join_2_id}" ("" "") [iterator]) ; behaviour = echo
                        )
                        (next iterator)
                    )
                )
            )
            (call "some_peer_id" ("" "") []) ;; (1)
        )
    "#);

    let executor = AirScriptExecutor::new(
        TestRunParameters::from_init_peer_id("set_variable_id"),
        vec![],
        vec![peer_1_id, peer_2_id].into_iter().map(Into::into),
        &script,
    )
    .expect("Invalid annotated AIR script");

    let _result = executor.execute_one(set_variable_id).unwrap();
    let _peer_1_result = executor.execute_one(peer_1_id).unwrap();
    let _peer_2_result = executor.execute_one(peer_2_id).unwrap();

    let _join_1_result = executor.execute_one(join_1_id).unwrap();
    let join_1_result = executor.execute_one(join_1_id).unwrap(); // before 0.20.9 it fails here
    let actual_trace = trace_from_result(&join_1_result);
    let expected_trace = vec![
        executed_state::scalar(json!([peer_1_id, peer_2_id])),
        executed_state::par(2, 3),
        executed_state::scalar_string(peer_1_value),
        executed_state::ap(0),
        executed_state::par(2, 0),
        executed_state::scalar_string(peer_2_value),
        executed_state::ap(1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(3, SubTraceDesc::new(8.into(), 4), SubTraceDesc::new(12.into(), 0)),
            executed_state::subtrace_lore(6, SubTraceDesc::new(12.into(), 4), SubTraceDesc::new(16.into(), 0)),
        ]),
        executed_state::par(3, 0),
        executed_state::par(1, 1),
        executed_state::scalar_string(peer_1_value),
        executed_state::request_sent_by(peer_1_id),
        executed_state::par(3, 0),
        executed_state::par(1, 1),
        executed_state::scalar_string(peer_2_value),
        executed_state::request_sent_by(peer_2_id),
        executed_state::request_sent_by(join_1_id),
    ];

    assert_eq!(actual_trace, expected_trace);
}
