use rand::{thread_rng, Rng};
use gtk4::{prelude::*, Orientation, SpinButton,Application, ApplicationWindow, Button, Label, Box, ToggleButton};
use arboard::Clipboard; 



fn main() {
   
    let _pass = generate_password(7, false);
    println!("pass: {}", _pass);


    let app = Application::builder()
        .application_id("com.password.generator")
        .build();
    app.connect_activate(build_ui);
    app.run();


   }

fn build_ui(app: &Application) {
    const MARGINS: i32= 12;
    
    let label = Label::builder()
        .label("your password")
        .margin_top(MARGINS)
        .margin_bottom(MARGINS)
        .margin_start(MARGINS)
        .margin_end(MARGINS)
        .build();

    let generate = Button::builder()
        .label("generate password")
        .margin_top(MARGINS)
        .margin_start(MARGINS)
        .margin_end(MARGINS)
        .margin_bottom(MARGINS)
        .build();
    
    let spin_button = SpinButton::new(
        Some(&gtk4::Adjustment::new(0.0,0.0,100.0,1.0,10.0,0.0)),
        1.0,
        1
    );
     
    let toggle_button = ToggleButton::builder()
        .label("Numbers")
        .margin_top(MARGINS)
        .margin_start(MARGINS)
        .margin_end(MARGINS)
        .margin_bottom(MARGINS)
        .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&label);
    content.append(&toggle_button);
    content.append(&spin_button);
    content.append(&generate);

    let window = ApplicationWindow::builder()
        .title("password generator")
        .application(app)
        .child(&content)
        .build();

       
    
    let cloned_spin = spin_button.clone();



    generate.connect_clicked(move |_| {
        let length = cloned_spin.value() as usize;
        let numbers = ToggleButtonExt::is_active(&toggle_button);
        let password = generate_password(length, numbers);
        println!("password: {password}");
        label.set_text(&password);
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(password).unwrap();
    });
    window.show();
}
 
fn generate_password(length: usize, numbers: bool) -> String {
    const NUM: usize = 3;
    if numbers && length < 3{
        return "invalid length".to_string();
    }
    let mut rng = thread_rng();
    let alphabet_chars: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let num_chars: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let only_num_chars: &[u8] = b"1234567890";
    let char_set = if numbers { num_chars } else { alphabet_chars };
    

    let random_string: String = (0..length)
        .map(|_| {
            let index = rng.gen_range(0..char_set.len());
            char_set[index] as char
        })
        .collect();
    
    if numbers {
        let new_string: String= random_string[0..random_string.len() - NUM].to_owned();
        let random_num: String = (0..NUM).map(|_| {
            let index = rng.gen_range(0..only_num_chars.len());
            only_num_chars[index] as char
        })
        .collect();
        let password = format!("{}{}",new_string,random_num);
        return password;
    } else {
        return random_string;
    }
}

