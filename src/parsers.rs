mod code_block;
mod quote_block;
mod strong;

use crate::utils::{Formatting, Parser};
use code_block::*;
use quote_block::*;
use strong::*;

#[derive(Debug)]
pub enum Format {
  Plain,
  Quote(QuoteBlock),
  Code(CodeBlock),
  Strong(Strong),
}

pub static PARSERS: [for<'r> fn(&'r str) -> std::vec::Vec<Formatting>; 3] =
  [CodeBlock::parse, QuoteBlock::parse, Strong::parse];
