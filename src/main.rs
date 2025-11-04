use arboard::Clipboard;
use std::error::Error;
use std::io;
use to_arabic_numerals::transformer_ar;
fn main() {
    if let Err(_) = modify_clipboard() {
        eprintln!("Error modifying clipboard,\neither it is empty or not accessible!");
        eprintln!("Switching to input mode, when you finish press <ctrl> and c at the same time.");
        if let Err(_) = modify_input(){
            eprintln!("Error getting input!");
            std::process::exit(1);
        }
    }
}

fn modify_clipboard() -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;
    let content = clipboard.get_text()?;
    let new_content = transformer_ar(content);
    clipboard.set_text(new_content)?;
    Ok(())
}
fn modify_input() -> Result<(), Box<dyn Error>>{
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        print!("{}", transformer_ar(input));
    }
}
