use crate::parsers::Format;
use crate::utils::{ContentRange, Formatting, Parser, Range};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct CodeBlock {
  language: String,
}

impl Parser for CodeBlock {
  fn parse(text: &str) -> Vec<Formatting> {
    lazy_static! {
      static ref RE: Regex =
        Regex::new(r"(?mU)^```(?P<language>.*)\n(?P<code>.*(?:\n.*)*)\n```").unwrap();
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
          content_ranges: vec![ContentRange {
            range: Range {
              start: mat.start() + language.len() + 4,
              length: content.len(),
            },
            children: vec![],
          }],
          format: Format::Code(CodeBlock {
            language: String::from(language),
          }),
          allows_subformatting: true,
        }
      })
      .collect::<Vec<Formatting>>()
  }

  fn allows_inner_format() -> bool {
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let ranges = CodeBlock::parse("test\n```js\ncode1\n```\n\ntest\n```xml\ncode2\n```\n\ntest");
    assert_eq!(ranges.len(), 2);
  }
}
