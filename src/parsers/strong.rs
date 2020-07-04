use crate::parsers::Format;
use crate::utils::{Formatting, Parser, Range};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
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
        // content_ranges: vec![Range {
        //   start: mat.start() + 1,
        //   length: mat.end() - mat.start() - 2,
        // }],
        format: Format::Strong(Strong {}),
        children: vec![Formatting {
          format: Format::Plain,
          children: vec![],
          range: Range {
            start: mat.start() + 1,
            length: mat.end() - mat.start() - 2,
          },
        }],
      })
      .collect::<Vec<Formatting>>()
  }
}
