use crate::parsers::Format;
use crate::utils::{Formatting, Parser, Range};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct QuoteBlock {}

impl Parser for QuoteBlock {
  fn parse(text: &str) -> Vec<Formatting> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"(?m)(?:^> (.+)$)(?:\n(?:^> (.+)$))*").unwrap();
    }
    RE.find_iter(text)
      .map(|mat| {
        let substr = &text[mat.start()..mat.end()];
        let captures = RE.captures(substr).unwrap();
        let language = &captures[1];
        let content = &captures[2];

        Formatting {
          range: Range {
            start: mat.start(),
            length: mat.end() - mat.start(),
          },
          content_ranges: vec![],
          format: Format::Quote(QuoteBlock {}),
          children: vec![],
        }
      })
      .collect::<Vec<Formatting>>()
  }
}
