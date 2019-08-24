pub enum Action {
    Add_Note,
    Arm,
    Edit_Mode,
    Loop_Mode,
    Pitch,
    Volume,
    Select_Y, // Yellow
    Select_G, // Green
    Select_P, // Pink
    Select_B, // Blue
}

struct Event {
    type: Action,
    payload: Option(),
}

struct ViewState {

}

fn root(state: &ViewState, action: Event) -> &ViewState {
    match action {
        Action::Add_Event,
        Action::
    }
    View
}