#[derive(PartialEq, Eq)]
pub enum OpenTab {
    Debug,
    Heroes,
    Console,
    About,
}


pub struct InspectorToggle(pub bool);
impl Default for InspectorToggle{
    fn default() -> InspectorToggle{
        InspectorToggle(false)
    }
}

#[derive(PartialEq)]
pub struct Console{
    pub string: String,
    pub history: Vec<String>,
    pub iter: usize,
}
impl Console{
    pub fn new() -> Console {
        Console{
            string: String::from(""),
            history: Vec::new(),
            iter: 0,
        }
    }
}

#[derive(PartialEq)]
pub struct Stats {
    pub fps: bool,
    pub tick: bool,
    pub state_list: bool,
    pub state: bool,
}
impl Stats {
    pub fn new() -> Stats {
        Stats {
            fps: true,
            tick: false,
            state: true,
            state_list: true,
        }
    }
}
