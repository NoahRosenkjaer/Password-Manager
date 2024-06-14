use rand::Rng;
use slint::SharedString;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_button_pressed({
        move || {
            let ui = ui_handle.unwrap();
            let length = ui.get_length() as u8;
            let password = pass_gen(length);
            ui.set_password(password);
        }
    });
    ui.run()
}

fn pass_gen(num: u8) -> SharedString {
    let char = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w",
        "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y",
        "Z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "!", "#", "$", "%", "&", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?",
        "@", "[", "]", "^", "_", "{", "|", "}", "~"];

    let mut tmp_pass = Vec::new();

    for _ in 0..num {
        tmp_pass.push(char[rand::thread_rng().gen_range(0..=87)]);
    }
    
    let string: String = tmp_pass.into_iter().collect();
    let shared_string: SharedString = SharedString::from(string);
    
    shared_string    
}