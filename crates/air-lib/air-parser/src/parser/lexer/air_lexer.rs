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

use super::errors::LexerError;
use super::token::Token;
use super::{AirPos, LexerResult};

use std::iter::Peekable;
use std::str::CharIndices;

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

pub struct AIRLexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Iterator for AIRLexer<'input> {
    type Item = Spanned<Token<'input>, AirPos, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'input> AIRLexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Spanned<Token<'input>, AirPos, LexerError>> {
        while let Some((start_pos, ch)) = self.chars.next() {
            let start_pos = AirPos::from(start_pos);
            match ch {
                '(' => return Some(Ok((start_pos, Token::OpenRoundBracket, start_pos + 1))),
                ')' => return Some(Ok((start_pos, Token::CloseRoundBracket, start_pos + 1))),

                '[' => return Some(Ok((start_pos, Token::OpenSquareBracket, start_pos + 1))),
                ']' => return Some(Ok((start_pos, Token::CloseSquareBracket, start_pos + 1))),

                ';' => self.skip_comment(),

                ch if ch.is_whitespace() => {}

                '"' => return self.tokenize_string_literal(start_pos),

                _ => return self.tokenize_string(start_pos, false),
            }
        }

        None
    }

    fn skip_comment(&mut self) {
        const NEW_LINE: char = '\n'; // TODO: consider '\n\r'

        for (_, ch) in &mut self.chars {
            if ch == NEW_LINE {
                break;
            }
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn tokenize_string_literal(
        &mut self,
        start_pos: AirPos,
    ) -> Option<Spanned<Token<'input>, AirPos, LexerError>> {
        for (pos, ch) in &mut self.chars {
            let pos = AirPos::from(pos);
            if ch == '"' {
                // + 1 to count an open double quote
                let string_size = pos - start_pos + 1;

                return Some(Ok((
                    start_pos,
                    Token::StringLiteral(&self.input[(start_pos + 1).into()..pos.into()]),
                    start_pos + string_size,
                )));
            }
        }

        Some(Err(LexerError::unclosed_quote(
            start_pos..self.input.len().into(),
        )))
    }

    #[allow(clippy::unnecessary_wraps)]
    fn tokenize_string(
        &mut self,
        start_pos: AirPos,
        open_square_bracket_met: bool,
    ) -> Option<Spanned<Token<'input>, AirPos, LexerError>> {
        let end_pos = self.advance_to_token_end(start_pos, open_square_bracket_met);

        // this slicing is safe here because borders come from the chars iterator
        let token_str = &self.input[start_pos.into()..end_pos.into()];

        let token = match string_to_token(token_str, start_pos) {
            Ok(token) => token,
            Err(e) => return Some(Err(e)),
        };

        let token_str_len = end_pos - start_pos;
        Some(Ok((start_pos, token, start_pos + token_str_len)))
    }

    fn advance_to_token_end(&mut self, start_pos: AirPos, square_met: bool) -> AirPos {
        let mut end_pos = start_pos;
        let mut round_brackets_balance: i64 = 0;
        let mut square_brackets_balance = i64::from(square_met);

        while let Some((pos, ch)) = self.chars.peek() {
            end_pos = (*pos).into();
            let ch = *ch;

            update_brackets_count(
                ch,
                &mut round_brackets_balance,
                &mut square_brackets_balance,
            );

            if should_stop(ch, round_brackets_balance, square_brackets_balance) {
                break;
            }

            self.chars.next();
        }

        self.advance_end_pos(&mut end_pos);
        end_pos
    }

    // if it was the last char, advance the end position.
    fn advance_end_pos(&mut self, end_pos: &mut AirPos) {
        if self.chars.peek().is_none() {
            *end_pos = self.input.len().into();
        }
    }
}

fn update_brackets_count(
    ch: char,
    round_brackets_balance: &mut i64,
    square_brackets_balance: &mut i64,
) {
    if ch == '(' {
        *round_brackets_balance += 1;
    } else if ch == ')' {
        *round_brackets_balance -= 1;
    } else if ch == '[' {
        *square_brackets_balance += 1;
    } else if ch == ']' {
        *square_brackets_balance -= 1;
    }
}

fn should_stop(ch: char, round_brackets_balance: i64, open_square_brackets_balance: i64) -> bool {
    ch.is_whitespace() || round_brackets_balance < 0 || open_square_brackets_balance < 0
}

fn string_to_token(input: &str, start_pos: AirPos) -> LexerResult<Token> {
    match input {
        "" => Err(LexerError::empty_string(start_pos..start_pos)),

        CALL_INSTR => Ok(Token::Call),
        CANON_INSTR => Ok(Token::Canon),
        AP_INSTR => Ok(Token::Ap),
        SEQ_INSTR => Ok(Token::Seq),
        PAR_INSTR => Ok(Token::Par),
        FAIL_INSTR => Ok(Token::Fail),
        FOLD_INSTR => Ok(Token::Fold),
        XOR_INSTR => Ok(Token::Xor),
        NEVER_INSTR => Ok(Token::Never),
        NEW_INSTR => Ok(Token::New),
        NEXT_INSTR => Ok(Token::Next),
        NULL_INSTR => Ok(Token::Null),
        MATCH_INSTR => Ok(Token::Match),
        MISMATCH_INSTR => Ok(Token::MisMatch),

        INIT_PEER_ID => Ok(Token::InitPeerId),
        _ if input.starts_with(LAST_ERROR) => parse_last_error(input, start_pos),
        TIMESTAMP => Ok(Token::Timestamp),
        TTL => Ok(Token::TTL),

        TRUE_VALUE => Ok(Token::Boolean(true)),
        FALSE_VALUE => Ok(Token::Boolean(false)),

        str => super::call_variable_parser::try_parse_call_variable(str, start_pos),
    }
}

fn parse_last_error(input: &str, start_pos: AirPos) -> LexerResult<Token<'_>> {
    let last_error_size = LAST_ERROR.len();
    if input.len() == last_error_size {
        return Ok(Token::LastError);
    }

    if input.len() <= last_error_size {
        return Err(LexerError::lambda_parser_error(
            start_pos + last_error_size..start_pos + input.len(),
            "lambda AST applied to last error has not enough size",
        ));
    }

    let last_error_accessor = crate::parse_lambda(&input[last_error_size..]).map_err(|e| {
        LexerError::lambda_parser_error(
            start_pos + last_error_size..start_pos + input.len(),
            e.to_string(),
        )
    })?;
    let last_error_token = Token::LastErrorWithLambda(last_error_accessor);

    Ok(last_error_token)
}

const CALL_INSTR: &str = "call";
const CANON_INSTR: &str = "canon";
const AP_INSTR: &str = "ap";
const SEQ_INSTR: &str = "seq";
const PAR_INSTR: &str = "par";
const FAIL_INSTR: &str = "fail";
const FOLD_INSTR: &str = "fold";
const XOR_INSTR: &str = "xor";
const NEVER_INSTR: &str = "never";
const NEW_INSTR: &str = "new";
const NEXT_INSTR: &str = "next";
const NULL_INSTR: &str = "null";
const MATCH_INSTR: &str = "match";
const MISMATCH_INSTR: &str = "mismatch";

const INIT_PEER_ID: &str = "%init_peer_id%";
const LAST_ERROR: &str = "%last_error%";
const TIMESTAMP: &str = "%timestamp%";
const TTL: &str = "%ttl%";

const TRUE_VALUE: &str = "true";
const FALSE_VALUE: &str = "false";
