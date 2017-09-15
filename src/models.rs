use rustbox::{Color, RustBox, RB_BOLD};

pub struct Model { 
    count: i32
}

pub struct ViewModel {
    pub model: Option<Model>
}

impl ViewModel {
    pub fn init(&mut self) {
        self.model = Some(Model {
            count: 0
        });
    }

    pub fn count_up(&mut self) {
        if let Some(ref mut model) = self.model {
            model.count += 1;
        }
    }

    pub fn count_down(&mut self) {
        if let Some(ref mut model) = self.model {
            model.count -= 1;
        }
    }

    pub fn render(&self, g: &RustBox) {
        if let Some(ref model) = self.model {
            g.print(1, 10, RB_BOLD, Color::White, Color::Black, &format!("VALUE: {}", model.count));
        }
    }
}
