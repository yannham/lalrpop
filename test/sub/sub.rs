use util::tok::Tok;

#[allow(non_snake_case)]
pub fn parse_S<TOKENS: IntoIterator<Item=Tok>>(
    tokens: TOKENS)
    -> Result<(Option<Tok>, ()), Option<Tok>>
{
    let mut tokens = tokens.into_iter();
    let lookahead = tokens.next();
    match try!(__parseS::__state0(lookahead, &mut tokens)) {
        (lookahead, __parseS::__Nonterminal::S(nt)) => Ok((lookahead, nt)),
        _ => unreachable!(),
    }
}

mod __parseS {
    #![allow(non_snake_case, unused_mut, unused_variables)]

    use util::tok::Tok;

    pub enum __Nonterminal {
        E(()),
        S(()),
        T(()),
    }

    // State 0
    //   S = (*) E [EOF]
    //   E = (*) E "-" T [EOF]
    //   E = (*) T [EOF]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T ["-"]
    //   T = (*) "Num" [EOF]
    //   T = (*) "(" E ")" [EOF]
    //   T = (*) "Num" ["-"]
    //   T = (*) "(" E ")" ["-"]
    //
    //   "Num" -> Shift(S4)
    //   "(" -> Shift(S3)
    //
    //   T -> S1
    //   E -> S2
    pub fn __state0<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Num(..)) => {
                let sym0 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state4(lookahead, tokens, sym0));
            }
            Some(Tok::LParen(..)) => {
                let sym0 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state3(lookahead, tokens, sym0));
            }
            _ => {
                return Err(lookahead);
            }
        }
        loop {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::T(nt) => {
                    let sym0 = &mut Some(nt);
                    result = try!(__state1(lookahead, tokens, sym0));
                }
                __Nonterminal::E(nt) => {
                    let sym0 = &mut Some(nt);
                    result = try!(__state2(lookahead, tokens, sym0));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
    }

    // State 1
    //   E = T (*) [EOF]
    //   E = T (*) ["-"]
    //
    //   EOF -> Reduce(E = T => ActionFn(2);)
    //   "-" -> Reduce(E = T => ActionFn(2);)
    //
    pub fn __state1<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<()>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action2(sym0);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action2(sym0);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 2
    //   S = E (*) [EOF]
    //   E = E (*) "-" T [EOF]
    //   E = E (*) "-" T ["-"]
    //
    //   EOF -> Reduce(S = E => ActionFn(0);)
    //   "-" -> Shift(S5)
    //
    pub fn __state2<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<()>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action0(sym0);
                return Ok((lookahead, __Nonterminal::S(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym1 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state5(lookahead, tokens, sym0, sym1));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 3
    //   T = "(" (*) E ")" [EOF]
    //   T = "(" (*) E ")" ["-"]
    //   E = (*) E "-" T [")"]
    //   E = (*) T [")"]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T ["-"]
    //   T = (*) "Num" [")"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "Num" ["-"]
    //   T = (*) "(" E ")" ["-"]
    //
    //   "Num" -> Shift(S9)
    //   "(" -> Shift(S7)
    //
    //   T -> S8
    //   E -> S6
    pub fn __state3<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Num(..)) => {
                let sym1 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state9(lookahead, tokens, sym1));
            }
            Some(Tok::LParen(..)) => {
                let sym1 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state7(lookahead, tokens, sym1));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym0.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::T(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state8(lookahead, tokens, sym1));
                }
                __Nonterminal::E(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state6(lookahead, tokens, sym0, sym1));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 4
    //   T = "Num" (*) [EOF]
    //   T = "Num" (*) ["-"]
    //
    //   EOF -> Reduce(T = "Num" => ActionFn(3);)
    //   "-" -> Reduce(T = "Num" => ActionFn(3);)
    //
    pub fn __state4<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            None => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 5
    //   E = E "-" (*) T [EOF]
    //   E = E "-" (*) T ["-"]
    //   T = (*) "Num" [EOF]
    //   T = (*) "(" E ")" [EOF]
    //   T = (*) "Num" ["-"]
    //   T = (*) "(" E ")" ["-"]
    //
    //   "(" -> Shift(S3)
    //   "Num" -> Shift(S4)
    //
    //   T -> S10
    pub fn __state5<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<()>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::LParen(..)) => {
                let sym2 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state3(lookahead, tokens, sym2));
            }
            Some(Tok::Num(..)) => {
                let sym2 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state4(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::T(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state10(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 6
    //   T = "(" E (*) ")" [EOF]
    //   T = "(" E (*) ")" ["-"]
    //   E = E (*) "-" T [")"]
    //   E = E (*) "-" T ["-"]
    //
    //   "-" -> Shift(S11)
    //   ")" -> Shift(S12)
    //
    pub fn __state6<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<()>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Minus(..)) => {
                let sym2 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state11(lookahead, tokens, sym1, sym2));
            }
            Some(Tok::RParen(..)) => {
                let sym2 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state12(lookahead, tokens, sym0, sym1, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 7
    //   T = "(" (*) E ")" [")"]
    //   T = "(" (*) E ")" ["-"]
    //   E = (*) E "-" T [")"]
    //   E = (*) T [")"]
    //   E = (*) E "-" T ["-"]
    //   E = (*) T ["-"]
    //   T = (*) "Num" [")"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "Num" ["-"]
    //   T = (*) "(" E ")" ["-"]
    //
    //   "(" -> Shift(S7)
    //   "Num" -> Shift(S9)
    //
    //   E -> S13
    //   T -> S8
    pub fn __state7<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::LParen(..)) => {
                let sym1 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state7(lookahead, tokens, sym1));
            }
            Some(Tok::Num(..)) => {
                let sym1 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state9(lookahead, tokens, sym1));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym0.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::E(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state13(lookahead, tokens, sym0, sym1));
                }
                __Nonterminal::T(nt) => {
                    let sym1 = &mut Some(nt);
                    result = try!(__state8(lookahead, tokens, sym1));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 8
    //   E = T (*) [")"]
    //   E = T (*) ["-"]
    //
    //   ")" -> Reduce(E = T => ActionFn(2);)
    //   "-" -> Reduce(E = T => ActionFn(2);)
    //
    pub fn __state8<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<()>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action2(sym0);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action2(sym0);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 9
    //   T = "Num" (*) [")"]
    //   T = "Num" (*) ["-"]
    //
    //   "-" -> Reduce(T = "Num" => ActionFn(3);)
    //   ")" -> Reduce(T = "Num" => ActionFn(3);)
    //
    pub fn __state9<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let nt = super::__action3(sym0);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 10
    //   E = E "-" T (*) [EOF]
    //   E = E "-" T (*) ["-"]
    //
    //   "-" -> Reduce(E = E, "-", T => ActionFn(1);)
    //   EOF -> Reduce(E = E, "-", T => ActionFn(1);)
    //
    pub fn __state10<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<()>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<()>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 11
    //   E = E "-" (*) T [")"]
    //   E = E "-" (*) T ["-"]
    //   T = (*) "Num" [")"]
    //   T = (*) "(" E ")" [")"]
    //   T = (*) "Num" ["-"]
    //   T = (*) "(" E ")" ["-"]
    //
    //   "(" -> Shift(S7)
    //   "Num" -> Shift(S9)
    //
    //   T -> S14
    pub fn __state11<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<()>,
        sym1: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::LParen(..)) => {
                let sym2 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state7(lookahead, tokens, sym2));
            }
            Some(Tok::Num(..)) => {
                let sym2 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state9(lookahead, tokens, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        while sym1.is_some() {
            let (lookahead, nt) = result;
            match nt {
                __Nonterminal::T(nt) => {
                    let sym2 = &mut Some(nt);
                    result = try!(__state14(lookahead, tokens, sym0, sym1, sym2));
                }
                _ => {
                    return Ok((lookahead, nt));
                }
            }
        }
        return Ok(result);
    }

    // State 12
    //   T = "(" E ")" (*) [EOF]
    //   T = "(" E ")" (*) ["-"]
    //
    //   "-" -> Reduce(T = "(", E, ")" => ActionFn(4);)
    //   EOF -> Reduce(T = "(", E, ")" => ActionFn(4);)
    //
    pub fn __state12<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<()>,
        sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            None => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 13
    //   T = "(" E (*) ")" [")"]
    //   T = "(" E (*) ")" ["-"]
    //   E = E (*) "-" T [")"]
    //   E = E (*) "-" T ["-"]
    //
    //   "-" -> Shift(S11)
    //   ")" -> Shift(S15)
    //
    pub fn __state13<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<()>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Minus(..)) => {
                let sym2 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state11(lookahead, tokens, sym1, sym2));
            }
            Some(Tok::RParen(..)) => {
                let sym2 = &mut lookahead;
                let lookahead = tokens.next();
                result = try!(__state15(lookahead, tokens, sym0, sym1, sym2));
            }
            _ => {
                return Err(lookahead);
            }
        }
        return Ok(result);
    }

    // State 14
    //   E = E "-" T (*) [")"]
    //   E = E "-" T (*) ["-"]
    //
    //   "-" -> Reduce(E = E, "-", T => ActionFn(1);)
    //   ")" -> Reduce(E = E, "-", T => ActionFn(1);)
    //
    pub fn __state14<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<()>,
        sym1: &mut Option<Tok>,
        sym2: &mut Option<()>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action1(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::E(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }

    // State 15
    //   T = "(" E ")" (*) [")"]
    //   T = "(" E ")" (*) ["-"]
    //
    //   ")" -> Reduce(T = "(", E, ")" => ActionFn(4);)
    //   "-" -> Reduce(T = "(", E, ")" => ActionFn(4);)
    //
    pub fn __state15<TOKENS: Iterator<Item=Tok>>(
        mut lookahead: Option<Tok>,
        tokens: &mut TOKENS,
        sym0: &mut Option<Tok>,
        sym1: &mut Option<()>,
        sym2: &mut Option<Tok>,
    ) -> Result<(Option<Tok>, __Nonterminal), Option<Tok>> {
        let mut result: (Option<Tok>, __Nonterminal);
        match lookahead {
            Some(Tok::RParen(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            Some(Tok::Minus(..)) => {
                let sym0 = sym0.take().unwrap();
                let sym1 = sym1.take().unwrap();
                let sym2 = sym2.take().unwrap();
                let nt = super::__action4(sym0, sym1, sym2);
                return Ok((lookahead, __Nonterminal::T(nt)));
            }
            _ => {
                return Err(lookahead);
            }
        }
    }
}
fn __action0(
    __0: (),
) {
    ()
}
fn __action1(
    __0: (),
    __1: Tok,
    __2: (),
) {
    ()
}
fn __action2(
    __0: (),
) {
    ()
}
fn __action3(
    __0: Tok,
) {
    ()
}
fn __action4(
    __0: Tok,
    __1: (),
    __2: Tok,
) {
    ()
}
