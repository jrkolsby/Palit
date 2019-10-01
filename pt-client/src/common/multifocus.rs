use crate::common::action::Action;

pub struct MultiFocus {
    node: u16,
    consumes: Action,
    outputs: Action,
}
