use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkOpt(pub i8);

impl MarkOpt {
    pub const _UNDO_RENDER: MarkOpt = MarkOpt(0);
    pub const DO_RENDER: MarkOpt = MarkOpt(1);

    pub fn do_render(&self) -> bool {
        self.0 == MarkOpt::DO_RENDER.0
    }
}