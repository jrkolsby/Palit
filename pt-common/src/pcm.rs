pub type Key = u8;
pub type Volume = f64;
pub type Offset = u32;
pub type Param = f32;

#[derive(Clone, Debug)]
pub struct Route {
    pub id: u16,
    pub patch: Vec<Anchor>,
}

#[derive(Clone, Debug)]
pub struct Anchor {
    pub index: u16,
    pub module_id: u16,
    pub name: String,
    pub input: bool,
}

#[derive(Clone, Debug)]
pub struct Module {
    pub id: u16,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Note {
    pub id: u16,
    pub t_in: Offset,
    pub t_out: Offset,
    pub note: Key,
    pub vel: Volume,
}