extern crate rustbox;

mod view;
use view::UI;

mod models;
use models::ViewModel;
use models::KeyMode;

use rustbox::Key;

static mut VM: ViewModel = ViewModel{ model: None };
static mut QUIT: bool = false;

fn normal_mode(key: rustbox::Key) {
    match key {
        Key::Char('q') => { unsafe { QUIT = true; } },
        Key::Char('k') => { unsafe { } },
        Key::Char('j') => { unsafe { } },
        _ => {}
    }
}

fn input_mode(key: rustbox::Key) {
}

fn main() {
    let mut u = UI::init();
    unsafe {
        VM.init();
        u.bind(&VM);
    }
    
    loop {
        unsafe {
            if QUIT {
                break;
            }
        }

        u.render();

        match u.poll() {
            Ok(rustbox::Event::KeyEvent(key)) => {
                unsafe {
                    match VM.current_mode() {
                        &KeyMode::Normal => {
                            normal_mode(key);
                        },
                        &KeyMode::Input => {
                            input_mode(key);
                        }
                    }
                }
            },
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}
