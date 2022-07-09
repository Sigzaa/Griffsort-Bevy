#[derive(PartialEq, Eq)]
pub enum OpenTab {
    Debug,
    Heroes,
    Console,
    Config,
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
#[derive(Default)]
pub struct Update{
    pub progress: f32,
    pub is_update: bool,
}

#[derive(PartialEq)]
pub struct Stats {
    pub fps: bool,
    pub tick: bool,
    pub state_list: bool,
    pub state: bool,
    pub net_buffer: bool,
}
impl Stats {
    pub fn new() -> Stats {
        Stats {
            fps: true,
            tick: false,
            state: true,
            state_list: true,
            net_buffer: false,
        }
    }
}
#[derive(PartialEq)]
pub struct Config {
    pub exit_on_del: bool,

}
impl Config {
    pub fn new() -> Config {
        Config {
            exit_on_del: true,
        }
    }
}

