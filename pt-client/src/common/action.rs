#[derive(Debug, Clone)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,

    SelectR,
    SelectB,
    SelectG,
    SelectY,
    SelectP,

    MidiEvent,
    RotaryEvent,

    Noop,

    Effect,
    Arm,
    Edit_Mode,
    Loop_Mode,
    Pitch,
    Volume,

    OpenProject(String)
}
