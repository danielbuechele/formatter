use crate::parsers::Format;
use crate::utils::{ContentRange, Formatting, Parser, Range};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
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

        Formatting {
          range: Range {
            start: mat.start(),
            length: mat.end() - mat.start(),
          },
          content_ranges: vec![ContentRange {
            // TODO
            range: Range {
              start: mat.start(),
              length: mat.end() - mat.start(),
            },
            children: vec![],
          }],
          format: Format::Quote(QuoteBlock {}),
          allows_subformatting: true,
        }
      })
      .collect::<Vec<Formatting>>()
  }
}
