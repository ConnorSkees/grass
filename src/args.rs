use std::collections::HashMap;
use std::iter::Peekable;

use crate::common::Pos;
use crate::error::SassResult;
use crate::scope::Scope;
use crate::selector::Selector;
use crate::utils::{
    devour_whitespace, devour_whitespace_or_comment, eat_ident, read_until_closing_paren,
    read_until_closing_quote, read_until_closing_square_brace,
};
use crate::value::Value;
use crate::Token;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct FuncArgs(pub Vec<FuncArg>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct FuncArg {
    pub name: String,
    pub default: Option<Vec<Token>>,
    pub is_variadic: bool,
}

impl FuncArgs {
    pub const fn new() -> Self {
        FuncArgs(Vec::new())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CallArgs(HashMap<CallArg, Vec<Token>>);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum CallArg {
    Named(String),
    Positional(usize),
}

impl CallArg {
    pub fn position(&self) -> SassResult<usize> {
        match self {
            Self::Named(..) => todo!(),
            Self::Positional(p) => Ok(*p),
        }
    }

    pub fn decrement(self) -> CallArg {
        match self {
            Self::Named(..) => self,
            Self::Positional(p) => Self::Positional(p - 1),
        }
    }
}

impl CallArgs {
    pub fn new() -> Self {
        CallArgs(HashMap::new())
    }

    /// Get argument by name
    ///
    /// Removes the argument
    pub fn get_named(
        &mut self,
        val: String,
        scope: &Scope,
        super_selector: &Selector,
    ) -> Option<SassResult<Value>> {
        match self.0.remove(&CallArg::Named(val)) {
            Some(v) => Some(Value::from_tokens(
                &mut v.into_iter().peekable(),
                scope,
                super_selector,
            )),
            None => None,
        }
    }

    /// Get a positional argument by 0-indexed position
    ///
    /// Removes the argument
    pub fn get_positional(
        &mut self,
        val: usize,
        scope: &Scope,
        super_selector: &Selector,
    ) -> Option<SassResult<Value>> {
        match self.0.remove(&CallArg::Positional(val)) {
            Some(v) => Some(Value::from_tokens(
                &mut v.into_iter().peekable(),
                scope,
                super_selector,
            )),
            None => None,
        }
    }

    pub fn get_variadic(self, scope: &Scope, super_selector: &Selector) -> SassResult<Vec<Value>> {
        let mut vals = Vec::new();
        let mut args = self
            .0
            .into_iter()
            .map(|(a, v)| Ok((a.position()?, v)))
            .collect::<SassResult<Vec<(usize, Vec<Token>)>>>()?;
        args.sort_by(|(a1, _), (a2, _)| a1.cmp(a2));
        for arg in args {
            vals.push(Value::from_tokens(
                &mut arg.1.into_iter().peekable(),
                scope,
                super_selector,
            )?);
        }
        Ok(vals)
    }

    pub fn decrement(self) -> Self {
        CallArgs(
            self.0
                .into_iter()
                .map(|(k, v)| (k.decrement(), v))
                .collect(),
        )
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
}

pub(crate) fn eat_func_args<I: Iterator<Item = Token>>(
    toks: &mut Peekable<I>,
    scope: &Scope,
    super_selector: &Selector,
) -> SassResult<FuncArgs> {
    let mut args: Vec<FuncArg> = Vec::new();

    devour_whitespace(toks);
    while let Some(Token { kind, .. }) = toks.next() {
        let name = match kind {
            '$' => eat_ident(toks, scope, super_selector)?,
            ')' => break,
            _ => todo!(),
        };
        let mut default: Vec<Token> = Vec::new();
        let mut is_variadic = false;
        devour_whitespace(toks);
        let kind = match toks.next() {
            Some(Token { kind, .. }) => kind,
            _ => todo!("unexpected eof"),
        };
        match kind {
            ':' => {
                devour_whitespace(toks);
                while let Some(tok) = toks.peek() {
                    match &tok.kind {
                        ',' => {
                            toks.next();
                            args.push(FuncArg {
                                name: name.replace('_', "-"),
                                default: Some(default),
                                is_variadic,
                            });
                            break;
                        }
                        ')' => {
                            args.push(FuncArg {
                                name: name.replace('_', "-"),
                                default: Some(default),
                                is_variadic,
                            });
                            break;
                        }
                        _ => {
                            let tok = toks.next().expect("we know this exists!");
                            default.push(tok)
                        }
                    }
                }
            }
            '.' => {
                if toks.next().ok_or("expected \".\".")?.kind != '.' {
                    return Err("expected \".\".".into());
                }
                if toks.next().ok_or("expected \".\".")?.kind != '.' {
                    return Err("expected \".\".".into());
                }
                devour_whitespace(toks);
                if toks.next().ok_or("expected \")\".")?.kind != ')' {
                    return Err("expected \")\".".into());
                }

                is_variadic = true;

                args.push(FuncArg {
                    name: name.replace('_', "-"),
                    default: Some(default),
                    is_variadic,
                });
                break;
            }
            ')' => {
                args.push(FuncArg {
                    name: name.replace('_', "-"),
                    default: if default.is_empty() {
                        None
                    } else {
                        Some(default)
                    },
                    is_variadic,
                });
                break;
            }
            ',' => args.push(FuncArg {
                name: name.replace('_', "-"),
                default: None,
                is_variadic,
            }),
            _ => {}
        }
        devour_whitespace(toks);
    }
    devour_whitespace(toks);
    if let Some(Token { kind: '{', .. }) = toks.next() {
    } else {
        todo!("expected `{{` after args")
    }
    Ok(FuncArgs(args))
}

pub(crate) fn eat_call_args<I: Iterator<Item = Token>>(
    toks: &mut Peekable<I>,
    scope: &Scope,
    super_selector: &Selector,
) -> SassResult<CallArgs> {
    let mut args: HashMap<CallArg, Vec<Token>> = HashMap::new();
    devour_whitespace_or_comment(toks)?;
    let mut name = String::new();
    let mut val: Vec<Token> = Vec::new();
    loop {
        match toks.peek().unwrap().kind {
            '$' => {
                toks.next();
                let v = eat_ident(toks, scope, super_selector)?;
                devour_whitespace_or_comment(toks)?;
                if toks.peek().unwrap().kind == ':' {
                    toks.next();
                    name = v;
                } else {
                    val.push(Token::new(Pos::new(), '$'));
                    val.extend(v.chars().map(|x| Token::new(Pos::new(), x)));
                    name.clear();
                }
            }
            ')' => {
                toks.next();
                return Ok(CallArgs(args));
            }
            _ => name.clear(),
        }
        devour_whitespace_or_comment(toks)?;

        while let Some(tok) = toks.next() {
            match tok.kind {
                ')' => {
                    args.insert(
                        if name.is_empty() {
                            CallArg::Positional(args.len())
                        } else {
                            CallArg::Named(name.replace('_', "-"))
                        },
                        val,
                    );
                    return Ok(CallArgs(args));
                }
                ',' => break,
                '[' => {
                    val.push(tok);
                    val.extend(read_until_closing_square_brace(toks));
                }
                '(' => {
                    val.push(tok);
                    val.extend(read_until_closing_paren(toks));
                }
                '"' | '\'' => {
                    val.push(tok);
                    val.extend(read_until_closing_quote(toks, tok.kind));
                }
                _ => val.push(tok),
            }
        }

        args.insert(
            if name.is_empty() {
                CallArg::Positional(args.len())
            } else {
                CallArg::Named(name.replace('_', "-"))
            },
            val.clone(),
        );
        val.clear();
        devour_whitespace(toks);

        if toks.peek().is_none() {
            return Ok(CallArgs(args));
        }
    }
}
