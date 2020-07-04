use crate::parsers::Format;

pub trait Parser {
  fn parse(text: &str) -> Vec<Formatting>;
  fn allows_inner_format() -> bool {
    true
  }
}

#[derive(Debug)]
pub struct Range {
  pub start: usize,
  pub length: usize,
}

impl Range {
  pub fn end(&self) -> usize {
    self.start + self.length
  }
}

#[derive(Debug)]
pub struct Formatting {
  pub range: Range,
  // pub content_ranges: Vec<Range>,
  pub children: Vec<Formatting>,
  pub format: Format,
}
