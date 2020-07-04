use crate::parsers::Format;
use crate::utils::{ContentRange, Formatting, Parser, Range};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Strong {}

impl Parser for Strong {
  fn parse(text: &str) -> Vec<Formatting> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"(?mU)\*(.+)\*").unwrap();
    }
    RE.find_iter(text)
      .map(|mat| Formatting {
        range: Range {
          start: mat.start(),
          length: mat.end() - mat.start(),
        },
        format: Format::Strong(Strong {}),
        content_ranges: vec![ContentRange {
          range: Range {
            start: mat.start() + 1,
            length: mat.end() - mat.start() - 2,
          },
          children: vec![],
        }],
        allows_subformatting: true,
      })
      .collect::<Vec<Formatting>>()
  }
}

#[cfg(test)]
mod tests {
  use crate::parsers::{Format, Strong};
  use crate::utils::{ContentRange, Formatting, Parser, Range};

  #[test]
  fn it_works() {
    let text = "Test *one* two";
    let res = Strong::parse(text);
    assert_eq!(res.len(), 1);
    assert_eq!(
      res[0],
      Formatting {
        range: Range {
          start: 5,
          length: 5,
        },
        content_ranges: vec![ContentRange {
          range: Range {
            start: 6,
            length: 3,
          },
          children: vec![],
        },],
        format: Format::Strong(Strong {}),
        allows_subformatting: true,
      }
    );
    println!("{:#?}", res);
  }
}
