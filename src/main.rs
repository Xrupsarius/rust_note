use std::{env, fs, io};
use std::fs::{File, OpenOptions};
use std::io::{Write};
use std::path::Path;
use std::process::exit;

static FAILED_TO_READ_LINE: &'static str = "Failed to read line.";

fn main() {
    println!("This is note example app.");
    println!("Input your name: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(FAILED_TO_READ_LINE);
    println!("Hello, {}", input);

    loop {
        main_menu();
        println!("Enter number: ");
        let mut menu_number = String::new();
        io::stdin().read_line(&mut menu_number).expect(FAILED_TO_READ_LINE);
        let menu_num: i8 = menu_number.trim().parse().expect("This is not a number");
        select_menu_num(menu_num);
    }
}

fn read_input() -> String {
    let mut text = String::new();
    io::stdout().flush().expect("Failed flush buffer");
    match io::stdin().read_line(&mut text) {
        Ok(_) => (),
        Err(error) => panic!("${FAILED_TO_READ_LINE} + {}", error),
    }
    if text.ends_with("\n") {
        text.pop();
    }
    text
}

fn select_menu_num(menu_num: i8) {
    if menu_num == 1 {
        create_new_note();
    } else if menu_num == 2 {
        update_note();
    } else if menu_num == 3 {
        get_all_notes().unwrap();
    } else if menu_num == 4 {
        remove_note();
    } else if menu_num == 5 {
        println!("Have a nice day.");
        exit(0);
    }
}

fn update_note() {
    println!("Enter note name: ");
    let note_name = read_input();
    if find_note(note_name.clone()) {
        println!("Enter what you want: ");
        let new_lines = read_input();
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(note_name)
            .unwrap();
        file.write_all(new_lines.as_bytes()).unwrap();
        println!("Write new lines successfully.");
    } else {
        println!("Note is not find. Is it exists?");
    }
}

fn remove_note() {
    println!("Enter file name:");
    let file_name = read_input();
    if find_note(file_name.clone()) {
        fs::remove_file(&file_name).expect("Failed to delete file");
        println!("{file_name} successfully deleted.");
    } else {
        println!("{file_name} is not found.");
    }
}

fn get_all_notes() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    match fs::read_dir(&current_dir) {
        Ok(entries) => {
            println!("Current dir {:?} contain files: ", current_dir.display());
            for entry in entries {
                match entry {
                    Ok(entry) => println!("{:?}", entry.file_name()),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}

fn find_note(note_name: String) -> bool {
    return Path::new(&note_name).exists();
}

fn create_new_note() {
    println!("Enter note name: ");
    let note_name = read_input();
    if find_note(note_name.clone()) {
        println!("Note is exist. Just update it.");
    } else {
        let mut file = File::create(&note_name).expect("Failed to create file.");
        println!("Enter what you want write to note: ");
        let note_string = read_input();
        file.write_all(note_string.as_ref()).expect("Error while write to file.");
    }
}

fn main_menu() {
    println!("What's next: ");
    println!("[1] Create new note");
    println!("[2] Update note");
    println!("[3] Get all notes");
    println!("[4] Remove note");
    println!("[5] Exit");
}
