use crate::display::clear_screen;
use crate::errors::AppError;
use crate::menu::{handle_choice, show_menu};
use std::io::{self, Write};

pub fn run_app() -> Result<(), AppError> {
    loop {
        clear_screen();
        show_menu();

        print!("Ваш вибір: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let choice = input.trim();

        if choice == "0" {
            println!("\nЗавершення роботи програми...");
            break;
        }

        clear_screen();
        println!("Зачекайте, збираю інформацію...\n");

        handle_choice(choice)?;

        println!();
        
    }

    Ok(())
}