use std::default::Default;
use rustbox;
use rustbox::{Color, RustBox};

use models::ViewModel;

pub struct UI<'a> {
    r: RustBox,
    vm: Option<&'a ViewModel>
}
impl<'a> UI<'a> {
    pub fn init() -> UI<'a> {
        let renderer = match RustBox::init(Default::default()) {
            Result::Ok(r) => r,
            Result::Err(e) => panic!("{}", e),
        };
        let ui = UI{ r: renderer, vm: None };
        return ui;
    }

    pub fn bind(&mut self, _m: &'a ViewModel) {
        self.vm = Some(_m);
    }

    pub fn render(&self) {
        self.r.clear();
        self.r.print(1, 1, rustbox::RB_REVERSE, Color::Default, Color::Black, &format!("DEBUG: WIDTH: {} HEIGHT: {}", self.r.width(), self.r.height()));
        if let Some(vm) = self.vm {
            vm.render(&self.r);
        }
        self.r.present();
    }

    pub fn poll(&self) -> rustbox::EventResult {
        self.r.poll_event(false)
    } 
}
