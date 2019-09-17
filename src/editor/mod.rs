use crate::*;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Cursor {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct Editor {
    pub(crate) lines: Vec<String>,
    pub(crate) cursor: Cursor,
}

pub trait Movement {
    fn calculate(&self, editor: &Editor, subcursor: &Cursor) -> Cursor;
}

pub trait Operation {
    fn perform(&self, editor: &mut Editor);
}

impl Editor {
    pub fn new<StringLike: AsRef<str>>(buf: StringLike) -> Editor {
        let cursor = Cursor::default();
        let lines = buf.as_ref().split("\n").map(String::from).collect();
        Editor { cursor, lines }
    }

    pub fn shift(&mut self, movement: &dyn Movement) {
        self.exec(&Move(movement));
    }

    pub fn goto(&mut self, cursor: Cursor) {
        self.cursor = cursor;
    }

    pub fn exec(&mut self, operation: &dyn Operation) {
        operation.perform(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEXT: &'static str =
        "hello world\nthis is some sample text\nto test out cursor math \nand other things";

    #[test]
    fn proper_lines() {
        let cursor = Cursor { col: 0, line: 0 };
        let lines = vec![
            String::from("hello world"),
            String::from("this is some sample text"),
            String::from("to test out cursor math "),
            String::from("and other things"),
        ];

        assert_eq!(Editor::new(TEXT), Editor { cursor, lines })
    }
}
