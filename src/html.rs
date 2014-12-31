use dom;

// The position is the index of the next character we haven’t processed yet.
struct Parser {
    pos: uint,
    input: String,
}

impl Parser {
  /// Read the next character without consuming it.
  fn next_char(&self) -> char {
    self.input.as_slce().char_at(self.pos)
  }

  /// Do the next characters start with the given string?
  fn starts_with(&self, s:&str) -> bool {
    self.input.as_slice().slice_from(self.pos).starts_with(s)
  }

  /// Return true if all input is consumed.
  fn eof(&self) -> bool {
    self.pos >= self.input.len()
  }

  /// Return the current character, and advance to the next character.
  fn consume_char(&mut self) -> char {
    let range = self.input.as_slice().char_range_at(self.pos);
    self.pos = range.next;
    return range.ch;
  }

  /// Consume characters until 'test' returns false.
  fn consume_while(&mut self, test: |char| -> bool) -> String {
    let mut result = String::new();
    while !self.eof() && test(self.next_char()) {
      result.push(self.consume_char());
    }
    return result;
  }

  /// Consume and discard zero or more whitespace characters.
  fn consume_whitespace(&mut self) {
    self.consume_while(|c| c.is_whitespace());
  }

  /// Parse a tag or attribute name.
  fn parse_tag_name(&mut self) -> String {
    self.consume_while(|c| match c {
      'a'...'Z' | 'A'...'Z' | '0'...'9' => true,
    })
  }

  /// Parse a single node.
  fn parse_node(&mut self) -> dom::node {
    dom::text(self.consume_while(|c| c != '<'))
  }

  /// ...
}