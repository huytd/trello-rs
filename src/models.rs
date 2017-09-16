use rustbox::{Color, RustBox, RB_BOLD, RB_REVERSE, RB_NORMAL};
use std::path::Path;
use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

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

    pub fn load_data(&mut self) {
        if let Some(ref mut model) = self.model {
            let mut path = env::home_dir().unwrap();
            path.push(".config/trello-rs/data");
            let file = File::open(path);
            match file {
                Ok(file) => {
                    let buf = BufReader::new(file);
                    for line in buf.lines() {
                        if let Some(line) = line.ok() {
                            if let Some(prefix) = line.get(0..4) {
                                if let Some(title) = line.get(5..) {
                                    let mut task = Task {
                                        status: TaskStatus::Backlog,
                                        title: format!("{}", title)
                                    };

                                    match prefix {
                                        "TODO" => task.status = TaskStatus::Backlog,
                                        "ONGO" => task.status = TaskStatus::Ongoing,
                                        "DONE" => task.status = TaskStatus::Done,
                                        _ => {}
                                    }

                                    model.tasks.push(task);
                                }
                            }
                        }
                    }
                },
                Err(err) => {
                    println!("ERR: {:?}", err);
                }
            }
        }
    }

    pub fn current_mode(&self) -> &KeyMode {
        if let Some(ref model) = self.model {
            return &model.mode;
        }
        &KeyMode::Normal
    }
    
    pub fn input_mode(&mut self) {
        if let Some(ref mut model) = self.model {
            model.mode = KeyMode::Input;
        }
    }

    pub fn normal_mode(&mut self) {
        if let Some(ref mut model) = self.model {
            model.mode = KeyMode::Normal;
            model.input = format!("");
        }
    }

    pub fn input_char(&mut self, c: char) {
        if let Some(ref mut model) = self.model {
            model.input = format!("{}{}", model.input, c);
        }
    }
    
    pub fn remove_char(&mut self) {
        if let Some(ref mut model) = self.model {
            model.input.pop();
        }
    }

    pub fn add_task(&mut self) {
        if let Some(ref mut model) = self.model {
            if model.input.len() > 0 {
                let task = Task{
                    status: TaskStatus::Backlog,
                    title: format!("{}", model.input)
                };
                model.tasks.push(task);
            }
        }
    }

    pub fn render(&self, g: &RustBox) {
        let screen_height = g.height();
        let screen_width = g.width();
        let section_height = screen_height / 2;
        let section_width = screen_width / 2;
        let mut backlog_task_count = 0;
        let mut ongoing_task_count = 0;
        let mut done_task_count = 0;

        g.print(1, screen_height - 1, RB_NORMAL, Color::Byte(8), Color::Black, "h,j,k,l: navigate | a: add | x: delete | m: change status | q: quit");

        if let Some(ref model) = self.model {
            g.print(1, 1, RB_REVERSE | RB_BOLD, Color::Byte(7), Color::Black, " ON GOING  ");
            g.print(section_width, 1, RB_REVERSE | RB_BOLD, Color::Byte(15), Color::Black, " TO DO     ");
            g.print(1, section_height, RB_REVERSE | RB_BOLD, Color::Byte(8), Color::Black, " DONE      ");

            for task in &model.tasks {
                match task.status {
                    TaskStatus::Backlog => {
                        backlog_task_count += 1;
                        g.print(section_width + 1, backlog_task_count + 2, RB_NORMAL, Color::Byte(7), Color::Black, &format!("{}. {}", backlog_task_count, task.title));
                    },
                    TaskStatus::Ongoing => {
                        ongoing_task_count += 1;
                        g.print(2, ongoing_task_count + 2, RB_NORMAL, Color::Byte(15), Color::Black, &format!("{}. {}", ongoing_task_count, task.title));
                    },
                    TaskStatus::Done => {
                        done_task_count += 1;
                        g.print(2, section_height + done_task_count + 1, RB_NORMAL, Color::Byte(8), Color::Black, &format!("{}. {}", done_task_count, task.title));
                    }
                }
            }

            match model.mode {
                KeyMode::Input => {
                    g.print(1, screen_height - 2, RB_NORMAL, Color::White, Color::Black, ">");
                    g.print(3, screen_height - 2, RB_NORMAL, Color::White, Color::Black, &format!("{}_", model.input));
                },
                KeyMode::Normal => {}
            }
        }
    }
}
