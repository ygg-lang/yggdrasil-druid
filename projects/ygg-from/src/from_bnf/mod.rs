use std::fmt::{Debug, Write};
use convert_case::{Case, Casing};

use pest_meta::ast::RuleType;
use pest_meta::optimizer::{OptimizedExpr, OptimizedRule};
use pest_meta::parse_and_optimize;
use yggdrasil_ir::grammar::GrammarInfo;
use yggdrasil_ir::rule::GrammarRule;
use crate::utils::Buffer;
use bnf::{Expression, Grammar, Production, Term};

pub struct BNFConverter {}


impl Default for BNFConverter {
    fn default() -> Self {
        Self {}
    }
}

impl BNFConverter {
    pub fn convert_bnf(&self, text: &str) -> anyhow::Result<String> {
        let grammar = text.parse::<Grammar>()?;
        let mut buffer = Buffer::new(self);
        for rule in grammar.productions_iter() {
            rule.build_ygg(&mut buffer)?;
        }
        Ok(buffer.finish())
    }
}

trait FromBNF {
    fn build_ygg(&self, f: &mut Buffer<BNFConverter>) -> std::fmt::Result;
    fn is_single(&self) -> bool {
        false
    }
}

impl<'i> FromBNF for Production {
    fn build_ygg(&self, f: &mut Buffer<BNFConverter>) -> std::fmt::Result {
        f.write_str("class ")?;
        match &self.lhs {
            Term::Terminal(v) => {
                f.write_str(&v.to_case(Case::Pascal))?
            }
            Term::Nonterminal(v) => { f.write_str(&v.to_case(Case::Pascal))? }
        };
        f.write_str(" {\n")?;
        for x in self.rhs_iter() {
            x.build_ygg(f)?;
        }
        f.write_str("}\n")
    }
}

impl<'i> FromBNF for Expression {
    fn build_ygg(&self, f: &mut Buffer<BNFConverter>) -> std::fmt::Result {
        for x in self.terms_iter() {
            match x {
                Term::Terminal(v) => {
                    write!(f, " {:?}", v.to_case(Case::Pascal))?
                }
                Term::Nonterminal(v) => {
                    write!(f, " {}", v.to_case(Case::Pascal))?
                }
            }
        }
        Ok(())
    }
}
