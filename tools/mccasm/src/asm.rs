pub struct Bank {
    pub data: [u8],
}

pub struct SourceReader {
    data: String,
    pos: usize,
}
impl SourceReader {
    pub fn from_str(data: String) -> Self {
        Self { data, pos: 0 }
    }
    pub fn get_pos(self) -> usize {
        self.pos
    }

    pub fn next_token(&mut self) -> &str {
        let start_pos = self.get_pos();
        for (i, character) in data[start_pos..].iter().enumerate() {
            if character.is_whitespace() {
                return &self.data[start_pos..i];
            }
        }
        &self.data[start_pos..self.get_pos()];
        loop {
            if let Some(character) = self.next_char() {
            } else {
                return;
            }
        }
    }
}

pub fn assemble(input: String) -> Vec<u8> {
    let out = Vec::<u8>::new();
    out
}
