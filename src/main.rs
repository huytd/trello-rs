extern crate rustbox;

mod view;
use view::UI;

mod models;
use models::ViewModel;

use rustbox::Key;

static mut VM: ViewModel = ViewModel{ model: None };

fn main() {
    let mut u = UI::init();
    unsafe {
        VM.init();
        u.bind(&VM);
    }
    
    loop {
        u.render();

        match u.poll() {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; },
                    Key::Char('k') => { unsafe { } },
                    Key::Char('j') => { unsafe { } },
                    _ => {}
                }
            },
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}
