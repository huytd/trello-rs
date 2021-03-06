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
        Key::Char('k') => { unsafe { VM.move_selection_up(); } },
        Key::Char('j') => { unsafe { VM.move_selection_down(); } },
        Key::Char('h') => { unsafe { VM.move_selection_left(); } },
        Key::Char('l') => { unsafe { VM.move_selection_right(); } },
        Key::Char('a') => { unsafe { VM.input_mode(); } },
        _ => {}
    }
}

fn input_mode(key: rustbox::Key) {
    match key {
        Key::Esc => { unsafe { VM.normal_mode(); } },
        Key::Backspace => { unsafe { VM.remove_char(); } },
        Key::Enter => { unsafe {
            VM.add_task();
            VM.normal_mode();
        } },
        Key::Char(c) => {
            unsafe {
                VM.input_char(c);
            }
        }
        _ => {}
    }
}

fn main() {
    let mut u = UI::init();
    unsafe {
        VM.init();
        VM.load_data();
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
                        &KeyMode::Normal => normal_mode(key),
                        &KeyMode::Input => input_mode(key)
                    }
                }
            },
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}
