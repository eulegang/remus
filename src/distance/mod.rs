use crate::{Cursor, Editor, Movement};

pub enum TextObject {
    Char,
    Word,
    Line,
    Paragraph,
}

pub struct Distance(pub i32, pub TextObject);

impl Movement for Distance {
    fn calculate(&self, editor: &Editor, subcursor: &Cursor) -> Cursor {
        use TextObject::*;

        let mut cursor = *subcursor;

        match self {
            Distance(mag, Char) => {
                window_add(&mut cursor.col, *mag, &editor.lines[cursor.line].len() - 1)
            }

            Distance(mag, Line) => window_add(&mut cursor.line, *mag, &editor.lines.len() - 1),

            Distance(mag, Word) => {}

            _ => unimplemented!(),
        }

        cursor
    }
}

#[inline]
fn window_add(value: &mut usize, add: i32, max: usize) {
    if add > 0 {
        if (add as usize) + *value > max {
            *value = max;
        } else {
            *value += add as usize
        }
    } else {
        let neg = -add as usize;

        if neg > *value {
            *value = 0;
        } else {
            *value -= neg;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use TextObject::*;

    const TEXT: &'static str =
        "hello world\nthis is some sample text\nto test out cursor math \nand other things";
    #[test]
    fn char_basic() {
        let editor = Editor::new(TEXT);

        assert_eq!(
            Distance(3, Char).calculate(&editor, &editor.cursor),
            Cursor { line: 0, col: 3 }
        );
    }

    #[test]
    fn char_overflow() {
        let editor = Editor::new(TEXT);

        assert_eq!(
            Distance(30, Char).calculate(&editor, &editor.cursor),
            Cursor { line: 0, col: 10 }
        );
    }

    #[test]
    fn char_underflow() {
        let mut editor = Editor::new(TEXT);
        editor.cursor = Cursor { line: 0, col: 5 };

        assert_eq!(
            Distance(-30, Char).calculate(&editor, &editor.cursor),
            Cursor { line: 0, col: 0 }
        );
    }

    #[test]
    fn line_basic() {
        let editor = Editor::new(TEXT);

        assert_eq!(
            Distance(2, Line).calculate(&editor, &editor.cursor),
            Cursor { line: 2, col: 0 }
        );
    }

    #[test]
    fn line_overflow() {
        let editor = Editor::new(TEXT);

        assert_eq!(
            Distance(50, Line).calculate(&editor, &editor.cursor),
            Cursor { line: 3, col: 0 }
        );
    }

    #[test]
    fn line_underflow() {
        let mut editor = Editor::new(TEXT);
        editor.cursor = Cursor { line: 2, col: 0 };

        assert_eq!(
            Distance(-50, Line).calculate(&editor, &editor.cursor),
            Cursor { line: 0, col: 0 }
        );
    }

    #[test]
    fn word_basic() {
        let mut editor = Editor::new(TEXT);
        editor.cursor = Cursor { line: 1, col: 9 };
    }

}
