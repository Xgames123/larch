use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use super::AsmError;
use crate::util::{parse_hex4};
use libnna::Op;
use libnna::{u4, ArgTy};


struct CodeParser<'a>{
    cur_linenum: usize,
    line: &'a str,
    line_iter: std::str::Lines<'a>,
    char_iter: std::str::CharIndices<'a>,
    span: Range<usize>,
    linenum: usize,
}
impl<'a> CodeParser<'a>{
    pub fn new(code: &'a str) -> Option<Self>{
        let mut line_iter = code.lines();
        let line = line_iter.next()?;
        let char_iter = line.char_indices();
        Some(Self{
            cur_linenum: 0,
            linenum: 0,
            line_iter,
            line,
            char_iter,
            span: 0..0,
        })

    }
    pub fn next_line(&mut self) -> Option<()>{
        self.cur_linenum+=1;
        self.line = self.line_iter.next()?;
        self.char_iter = self.line.char_indices();
        Some(())
    }
    pub fn next_char(&mut self) -> Option<(usize, char)> {
        loop{
            match self.char_iter.next(){
                Some(char)=>{
                    if char.1 == ';'{
                        self.next_line()?;
                    }else{
                        return Some(char)
                    }
                },
                None=>{
                    self.next_line()?
                }
            }
        }
    }
    pub fn linenum(&self) -> usize{
        self.linenum
    }
    pub fn span(&self) -> Range<usize>{
        self.span.clone()
    }
    pub fn last_location(&self) -> (usize, Range<usize>){
        (self.linenum(), self.span())
    }
}
impl<'a> Iterator for CodeParser<'a>{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        loop{
            let (index, char) = self.next_char()?;
            if !char.is_whitespace(){
                self.span.start = index;
                break;
            }
        }
        loop{
            let (index, char) = self.next_char()?;
            if char.is_whitespace(){
                self.span.end = index;
                self.linenum = self.cur_linenum;
                return Some(&self.line[self.span.clone()])
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

struct LexContext<'a>{
    data: &'a str,
    parser: RefCell<CodeParser<'a>>,
    filename: Rc<str>,
}
impl<'a> LexContext<'a>{
    pub fn err(&self, message: String) -> AsmError{
        AsmError { filename: self.filename.clone(), file: self.data, location: self.parser.borrow().last_location(), message }
    }
    pub fn next(&self) -> Option<&'a str>{
        self.parser.borrow_mut().next()
    }
    pub fn last_location(&self) -> (usize, Range<usize>) {
        self.parser.borrow().last_location()
    }
    pub fn expect_next(&self, message: String) -> Result<&'a str, AsmError>{
        self.parser.borrow_mut().next().ok_or_else(||{
            self.err(message)
        })
    }
}

pub fn parse_hex8<'a>(str: &'a str) -> Option<u8> {
    let str = str.to_lowercase();
    if str.len() != 2 {
        return None;
    }

    u8::from_str_radix(&str, 16).ok()
}

fn parse_compiler_directive<'a>(token: &'a str,ctx: &'a LexContext) -> Result<Token, AsmError<'a>>{
    match token {
        "org" => {
            let token = ctx.expect_next("Expected an 8 bit constant value after this. Ex. 1d".to_string())?;
            let addr = parse_hex8(token).ok_or(ctx.err("Expected an 8 bit constant value. Ex. 1d".to_string()))?;
            return Ok(Token::Org(addr));
        }
        _=>{
            return Err(ctx.err("Unknown compiler directive".to_string()));
        }
    }
}

fn parse_op<'a>(token: &'a str, ctx: &'a LexContext) -> Result<Token, AsmError<'a>>{
    let op = Op::try_from_str(token).ok_or(ctx.err("Unknown operation".to_string()))?;
}

pub fn parse_lex(input: &str, filename: Rc<str>) -> Result<Vec<LocatedToken>, AsmError> {
    let mut out_vec = Vec::new();
    let Some(mut parser) = CodeParser::new(input) else{
        return Ok(out_vec)
    };
    let ctx = LexContext{
        filename,
        parser: parser.into(),
        data: input,
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
