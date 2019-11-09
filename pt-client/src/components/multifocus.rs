use crate::common::action::Action;

pub struct FocusState {
    focus: (usize, usize),
    active: Action,
}

/*

    A multifocus render will render five components prefixed with
    a write_bg and write_fg method. Note that this will require making
    components bg and fg independent. A view will keep a 2D array of 
    MultiFocus objects which will be navigated when the view receives
    Up Down Left Right actions. If we reach the border of a list and try
    to exceed it, the view should pass a default action back up to the
    parent containing the direction to navigate. 

*/

pub struct MultiFocus {
    node: u16,
    consumes: Action,
    outputs: Action,
}
