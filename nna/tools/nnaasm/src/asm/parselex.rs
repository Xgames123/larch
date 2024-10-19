use std::borrow::Cow;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use super::AsmError;
use crate::util::{parse_hex4};
use libnna::Op;
use libnna::{u4, ArgTy};


struct CodeParser<'a>{
    cur_linenum: usize,
    code: &'a str,
    cur_index: usize,
    char_iter: std::str::CharIndices<'a>,
    last_location: (usize, Range<usize>),
}
impl<'a> CodeParser<'a>{
    pub fn new(code: &'a str) -> Option<Self>{
        Some(Self{
            last_location: (0, 0..0),
            code,
            cur_linenum: 0,
            cur_index: 0,
            char_iter: code.char_indices(),
        })

    }
    pub fn skip_line(&mut self){
        loop{
            match self.next_char() {
                None=>{return;}
                Some((_, char))=>{
                    if char == '\n'{
                        return;
                    }
                }
            }
        }
    }
    pub fn next_char(&mut self) -> Option<(usize, char)> {
        let (index, char) = self.char_iter.next()?;
        self.cur_index+=1;
        if char == '\n'{
            self.cur_linenum+=1;
            self.cur_index=0;
        }
        Some((index, char))
    }
    pub fn code(&self) -> &'a str{
        self.code
    }
    pub fn location(&self) -> (usize, Range<usize>){
        self.last_location.clone()
    }
    pub fn next_or_err(&mut self, message: Cow<'static, str>) -> Result<&'a str, LexError>{
        self.next().ok_or(LexError{message, location: self.last_location.clone()})
    }
}
impl<'a> Iterator for CodeParser<'a>{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let mut range_start = 0;
        let mut start_i = None;
        loop{
            let (index, char) = self.next_char()?;
            match start_i {
                None => {
                    if !char.is_whitespace(){
                        range_start = index;
                        start_i = Some(self.cur_index);
                    }

                }
                Some(start_i) => {
                    if char.is_whitespace(){
                        let loc = (self.cur_linenum, range_start..self.cur_index);
                        self.last_location = loc.clone();
                        return Some(&self.code[start_i..index])
                    }

                }
            }

        }

    }
}

#[derive(Debug, Clone)]
pub enum Token {
    LabelDef(Box<str>),
    LabelRef { name: Box<str>, wide: bool },
    Org(u8),
    Const(u4),
    Op(u8),
}
#[derive(Debug)]
pub struct LocatedToken {
    pub location: (usize, Range<usize>),
    pub token: Token,
}

struct LexError{
    location: (usize, Range<usize>),
    message: Cow<'static, str>,
}
impl LexError{
    pub fn new(message: Cow<'static, str>, location: (usize, Range<usize>)) -> Self{
        Self { location, message }
    }
    pub fn from_static(message: &'static str, location: (usize, Range<usize>)) -> Self{
        Self{
            location,
            message: Cow::Borrowed(message),
        }
    }
}

pub fn parse_hex8<'a>(str: &'a str) -> Option<u8> {
    let str = str.to_lowercase();
    if str.len() != 2 {
        return None;
    }

    u8::from_str_radix(&str, 16).ok()
}

fn parse_compiler_directive<'a>(token: &'a str,parser: &mut CodeParser) -> Result<Token, LexError>{
    match token {
        "org" => {
            let token = parser.next_or_err(Cow::Borrowed("Expected an 8 bit constant value after this. Ex. 1d"))?;
            let addr = parse_hex8(token).ok_or(LexError::from_static("Expected an 8 bit constant value. Ex. 1d", parser.location()))?;
            return Ok(Token::Org(addr));
        }
        _=>{
            return Err(LexError::from_static("Unknown compiler directive", parser.location()));
        }
    }
}

fn parse_op<'a>(token: &'a str, parser: &mut CodeParser) -> Result<Token, LexError>{
    let op = Op::try_from_str(token).ok_or(LexError::from_static("Unknown operation", parser.location()))?;
    Ok(Token::Op(match op.arg_types() {
        (ArgTy::None(), ArgTy::None()) => {
            op.opcode()
        },
        (ArgTy::None(), ArgTy::Any(arg_name)) => {
            let token = parser.next_or_err(Cow::Owned(format!("Expected a constant argument. for opeation: {}", op)));
            op.opcode()
        }
    }))
}

pub fn parse_lex(input: &str, filename: Rc<str>) -> Result<Vec<LocatedToken>, AsmError> {
    let mut out_vec = Vec::new();
    let Some(mut parser) = CodeParser::new(input) else{
        return Ok(out_vec)
    };
    let ctx = LexContext{
        filename,
        parser: parser.into(),
        code: input,
    };

    let push_token = |token: Token|{
        out_vec.push(LocatedToken { location: ctx.last_location(), token });
    };

    loop{
        let Some(mut token) = parser.next() else {
            return Ok(out_vec);
        };

        if token.starts_with('.'){
            parse_compiler_directive(&token[1..], &mut ctx)?;
        }

        parse_op(token, &ctx)?;
    }

    for (linenum, line) in input.lines().enumerate() {
        let linenum = linenum + 1;

        // strip comments
        if let Some(pos) = line.find('#') {
            line = &line[..pos];
        }
        if let Some(pos) = line.find(';') {
            line = &line[..pos];
        }

        let mut iter = line.split_whitespace();
        loop{
            let Some(token) = iter.next() else {break;};
            if token == ".org" {
                let token = iter.next().ok_or(AsmError{
                    linenum: linenum,
                })?;
                push_tok!(
                    Token::Org(parse_hex8(&token).ok_or_else(|| AsmError {
                        linenum: Some(linenum),
                        code_snip: token.into(),
                        message: "Failed to parse hex digit".into(),
                        stage: Stage::Parse,
                    })?)
                );
                continue;
            }

        }

        for token in line.split_whitespace() {
            if org {
                continue;
            }

            if token.ends_with(":") {
                push_tok!(Token::LabelDef(token[..token.len() - 1].into()), linenum);
                continue;
            }

            if token.starts_with("&&") {
                push_tok!(
                    Token::LabelRef {
                        name: token[2..].into(),
                        wide: true
                    },
                    linenum
                );
                continue;
            }
            if token.starts_with("&") {
                push_tok!(
                    Token::LabelRef {
                        name: token[1..].into(),
                        wide: false
                    },
                    linenum
                );
                continue;
            }
            if token.starts_with("0x") {
                push_tok!(
                    Token::Const(parse_hex4(&token[2..]).ok_or_else(|| AsmError {
                        linenum: Some(linenum),
                        code_snip: token.into(),
                        message: "Failed to parse hex digit".into(),
                        stage: Stage::Parse,
                    })?),
                    linenum
                );
                continue;
            }

            match Op::try_from_str(&token) {
                Some(op) => {
                    push_tok!(Token::Const(u4::from_low(op.opcode())), linenum);

                    let arg = op.arg_types().0;
                    match arg {
                        ArgTy::Any(),
                        ArgTy::Reg(),
                        ArgTy::None(),
                    }
                }
                None => {
                    return Err(AsmError {
                        linenum: Some(linenum),
                        code_snip: token.into(),
                        message: "Invalid operation".into(),
                        stage: Stage::Parse,
                    })
                }
            }
        }
    }

    Ok(vec)
}
