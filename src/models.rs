use rustbox::{Color, RustBox, RB_BOLD};

enum TaskStatus {
    Backlog,
    Ongoing,
    Blocked,
    Done
}

struct Task {
    status: TaskStatus,
    title: String
}

pub struct Model { 
    tasks: Vec<Task>
}

pub struct ViewModel {
    pub model: Option<Model>
}

impl ViewModel {
    pub fn init(&mut self) {
        self.model = Some(Model {
            tasks: vec![]
        });
    }

    pub fn render(&self, g: &RustBox) {
        if let Some(ref model) = self.model {
        }
    }
}
