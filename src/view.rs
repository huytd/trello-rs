use std::default::Default;
use rustbox;
use rustbox::{Color, RustBox, OutputMode};

use models::ViewModel;

pub struct UI<'a> {
    r: RustBox,
    vm: Option<&'a ViewModel>
}
impl<'a> UI<'a> {
    pub fn init() -> UI<'a> {
        let mut renderer = match RustBox::init(Default::default()) {
            Result::Ok(r) => r,
            Result::Err(e) => panic!("{}", e),
        };
        renderer.set_output_mode(OutputMode::EightBit);
        let ui = UI{ r: renderer, vm: None };
        return ui;
    }

    pub fn bind(&mut self, _m: &'a ViewModel) {
        self.vm = Some(_m);
    }

    pub fn render(&self) {
        self.r.clear();
        if let Some(vm) = self.vm {
            vm.render(&self.r);
        }
        self.r.present();
    }

    pub fn poll(&self) -> rustbox::EventResult {
        self.r.poll_event(false)
    } 
}
