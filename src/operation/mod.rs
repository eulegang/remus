use crate::Movement;

use crate::{Editor, Operation};

pub enum LinePosition {
    Absolute,
    Relative,
    Inplace,
}

pub struct Move<'a>(pub &'a dyn Movement);

impl<'a> Operation for Move<'a> {
    fn perform(&self, editor: &mut Editor) {
        editor.cursor = self.0.calculate(editor, &editor.cursor);
    }
}

/*
pub enum Op<'a> {
    Move(&'a dyn Movement),
    Yank(&'a dyn Movement),
    Delete(&'a dyn Movement),
    Insert(String, LinePosition),
    Append(String, LinePosition),
}*/
