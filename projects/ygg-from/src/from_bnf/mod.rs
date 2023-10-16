use std::fmt::{Debug, Write};
use convert_case::{Case, Casing};

use pest_meta::ast::RuleType;
use pest_meta::optimizer::{OptimizedExpr, OptimizedRule};
use pest_meta::parse_and_optimize;
use yggdrasil_ir::grammar::GrammarInfo;
use yggdrasil_ir::rule::GrammarRule;
use crate::utils::Buffer;
use bnf::{Expression, Grammar, Production, Term};
use ebnf::Node;

pub struct BNFConverter {}


impl Default for BNFConverter {
    fn default() -> Self {
        Self {}
    }
}

impl BNFConverter {
    pub fn convert_bnf(&self, grammar: &str) -> anyhow::Result<String> {
        let input = grammar.parse::<Grammar>()?;
        let mut buffer = Buffer::new(self);
        for rule in input.productions_iter() {
            rule.build_ygg(&mut buffer)?;
        }
        Ok(buffer.finish())
    }
    pub fn convert_ebnf(&self, grammar: &str) -> anyhow::Result<String> {
        let input = ebnf::get_grammar(grammar).unwrap();
        let mut buffer = Buffer::new(self);
        for rule in input.expressions {
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
        for (index, x) in self.rhs_iter().enumerate() {
            if index != 0 {
                f.write_str("  | ")?;
            }
            x.build_ygg(f)?;
            f.write_str("\n")?;
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

impl<'i> FromBNF for ebnf::Expression {
    fn build_ygg(&self, f: &mut Buffer<BNFConverter>) -> std::fmt::Result {
        f.write_str("class ")?;
        f.write_str(&self.lhs.to_case(Case::Pascal))?;
        f.write_str(" {\n")?;
        self.rhs.build_ygg(f)?;
        f.write_str("}\n")
    }
}

impl<'i> FromBNF for Node {
    fn build_ygg(&self, f: &mut Buffer<BNFConverter>) -> std::fmt::Result {
        match self {
            Node::String(v) => {
                writeln!(f, "{:?}", v)
            }
            Node::RegexString(v) => { writeln!(f, "{:?}", v) }
            Node::Terminal(v) => { writeln!(f, "{:?}", v) }
            Node::Multiple(v) => { writeln!(f, "{:?}", v) }
            Node::RegexExt(v, _) => { writeln!(f, "{:?}", v) }
            Node::Symbol(v, _, _) => { writeln!(f, "{:?}", v) }
            Node::Group(v) => { writeln!(f, "{:?}", v) }
            Node::Optional(v) => { writeln!(f, "{:?}", v) }
            Node::Repeat(v) => { writeln!(f, "{:?}", v) }
            Node::Unknown => { writeln!(f, "UNKNOWN") }
        }
    }
}
