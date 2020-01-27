use crate::tokenizer::Token;
use std::iter::Peekable;

// Abstract Syntax Tree
#[derive(Debug)]
pub enum AST {
    Add(Box<AST>, Box<AST>),
    Subtract(Box<AST>, Box<AST>),
    Multiply(Box<AST>, Box<AST>),
    Divide(Box<AST>, Box<AST>),
    Negate(Box<AST>),
    Call(String, Vec<AST>),
    Number(f64),
    Identifier(String),
}

// All errors from the parser are of type ParseError
#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedToken(Token),
}

// Some macro's for expecting and accepting tokens
// token MUST match
macro_rules! expect_token {
    ($iter:expr, $pattern:pat => $value:expr) => {
        match $iter.next().ok_or(ParseError::UnexpectedEOF)? {
            $pattern => $value,
            token => return Err(ParseError::UnexpectedToken(token)),
        }
    };
    ($iter:expr, $pattern:pat) => {
        expect_token!($iter, $pattern => ());
    };
}

// token MAY match, if it matches, consume it.
macro_rules! accept_token {
    ($iter:expr, $pattern:path as $value:ident $body:block) => {
        if let Some($pattern(_)) = $iter.peek() {
            let $value = match $iter.next().unwrap() {
                $pattern(x) => x,
                _ => panic!(),
            };
            $body;
        }
    };
    ($iter:expr, $pattern:path $body:block) => {
        if let Some(accept_token_val) = $iter.peek() {
            match accept_token_val {
                $pattern => {
                    $iter.next();
                    $body;
                }
                _ => (),
            };
        }
    };
}

// Actual parsing. This parser is a Recursive Descent Parser (RDP), which
// poses some restrictions on the grammar but for such a small language it
// doesn't matter.

// LL(1) grammar for calculator

// expr = additive-expr;
// additive-expr = multiplicative-expr (("+"|"-") multiplicative-expr)*;
// multiplicative-expr = unary-expr (("*"|"/") unary-expr)*;
// unary-expr = "-" unary-expr
//            | primary-expr;
// primary-expr = NUMBER
//              | var-or-call
//              | "(" expr ")";
// var-or-call = IDENTIFIER ( "(" (expr ("," expr)*)? ")" )?;


pub fn parse_expr<T>(iter: &mut Peekable<T>) -> Result<AST, ParseError>
where
    T: Iterator<Item = Token>,
{
    parse_additive_expr(iter)
}

pub fn parse_additive_expr<T>(iter: &mut Peekable<T>) -> Result<AST, ParseError>
where
    T: Iterator<Item = Token>,
{
    let mut a = parse_multiplicative_expr(iter)?;

    let mut repeating = true;
    while repeating {
        repeating = false;
        accept_token!(iter, Token::Plus {
            repeating = true;
            let b = parse_multiplicative_expr(iter)?;
            a = AST::Add(Box::new(a),Box::new(b));
        });
        if !repeating {
            accept_token!(iter, Token::Minus {
                repeating = true;
                let b = parse_multiplicative_expr(iter)?;
                a = AST::Subtract(Box::new(a),Box::new(b));
            });
        }
    }
    Ok(a)
}

pub fn parse_multiplicative_expr<T>(iter: &mut Peekable<T>) -> Result<AST, ParseError>
where
    T: Iterator<Item = Token>,
{
    let mut a = parse_unary_expr(iter)?;

    let mut repeating = true;
    while repeating {
        repeating = false;
        accept_token!(iter, Token::Star {
            repeating = true;
            let b = parse_unary_expr(iter)?;
            a = AST::Multiply(Box::new(a),Box::new(b));
        });
        if !repeating {
            accept_token!(iter, Token::Slash {
                repeating = true;
                let b = parse_unary_expr(iter)?;
                a = AST::Divide(Box::new(a),Box::new(b));
            });
        }
    }
    Ok(a)
}

// Parse unary-expr (see grammar documentation)
pub fn parse_unary_expr<T>(iter: &mut Peekable<T>) -> Result<AST, ParseError>
where
    T: Iterator<Item = Token>,
{
    accept_token!(iter, Token::Minus {
        return Ok(AST::Negate(Box::new(parse_unary_expr(iter)?)));
    });
    parse_primary_expr(iter)
}

// Parse primary-expr (see grammar documentation)
pub fn parse_primary_expr<T>(iter: &mut Peekable<T>) -> Result<AST, ParseError>
where
    T: Iterator<Item = Token>,
{
    accept_token!(iter, Token::Number as n {
        return Ok(AST::Number(n));
    });
    accept_token!(iter, Token::LParen {
        let val = parse_expr(iter);
        expect_token!(iter, Token::RParen);
        return val;
    });

    parse_var_or_call(iter)
}

// Parse var-or-call (see grammar documentation)
pub fn parse_var_or_call<T>(iter: &mut Peekable<T>) -> Result<AST, ParseError>
where
    T: Iterator<Item = Token>,
{
    let identifier = expect_token!(iter, Token::Identifier(id) => id);

    accept_token!(iter, Token::LParen {
        let mut params: Vec<AST> = Vec::new();
        accept_token!(iter, Token::RParen { return Ok(AST::Call(identifier,params)) });
        loop {
            params.push(parse_expr(iter)?);
            accept_token!(iter, Token::RParen { return Ok(AST::Call(identifier,params)) });
            expect_token!(iter, Token::Comma);
        }

    });

    Ok(AST::Identifier(identifier))
}
