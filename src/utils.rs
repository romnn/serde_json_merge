#[derive(Debug)]
pub struct Split<'r, 't> {
    finder: fancy_regex::Matches<'r, 't>,
    last: usize,
}

impl<'r, 't> Split<'r, 't> {
    pub fn new(finder: fancy_regex::Matches<'r, 't>) -> Self {
        Self { finder, last: 0 }
    }
}
impl<'t> Iterator for Split<'_, 't> {
    type Item = &'t str;

    fn next(&mut self) -> Option<&'t str> {
        let text = self.finder.text();
        match self.finder.next() {
            None | Some(Err(_)) => {
                if self.last > text.len() {
                    None
                } else {
                    let s = &text[self.last..];
                    self.last = text.len() + 1; // Next call will return None
                    Some(s)
                }
            }
            Some(Ok(m)) => {
                let matched = &text[self.last..m.start()];
                self.last = m.end();
                Some(matched)
            }
        }
    }
}

#[cfg(test)]
pub mod test {}
