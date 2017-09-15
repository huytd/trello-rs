use rustbox::{Color, RustBox, RB_BOLD, RB_REVERSE};

enum TaskStatus {
    Backlog,
    Ongoing,
    Done
}

struct Task {
    status: TaskStatus,
    title: String
}

pub enum KeyMode {
    Normal,
    Input
}

pub struct Model { 
    tasks: Vec<Task>,
    input: String,
    mode: KeyMode
}

pub struct ViewModel {
    pub model: Option<Model>
}

impl ViewModel {
    pub fn init(&mut self) {
        self.model = Some(Model {
            tasks: vec![],
            input: format!(""),
            mode: KeyMode::Normal
        });
    }

    pub fn current_mode(&self) -> &KeyMode {
        if let Some(ref model) = self.model {
            return &model.mode;
        }
        &KeyMode::Normal
    }

    pub fn render(&self, g: &RustBox) {
        let screen_height = g.height();
        let screen_width = g.width();
        let section_height = screen_height / 2;
        let section_width = screen_width / 2;

        if let Some(ref model) = self.model {
            g.print(1, 1, RB_REVERSE | RB_BOLD, Color::White, Color::Black, " ON GOING  ");
            g.print(section_width, 1, RB_REVERSE | RB_BOLD, Color::White, Color::Black, " TO DO     ");
            g.print(1, section_height, RB_REVERSE | RB_BOLD, Color::White, Color::Black, " DONE      ");
        }
    }
}
