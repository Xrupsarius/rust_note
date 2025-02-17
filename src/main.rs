use std::{fs, io};
use std::process::exit;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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

fn select_menu_num(menu_num: i8) {
    if menu_num == 1 {
        create_new_note();
    } else if menu_num == 2 {
        update_note();
    } else if menu_num == 3 {
        get_all_notes();
    } else if menu_num == 4 {
        remove_note();
    } else if menu_num == 5 {
        println!("Have a nice day.");
        exit(0);
    }
}

fn update_note() {
    println!("update_note fn");
}

fn remove_note() {
    println!("Enter file name:");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect(FAILED_TO_READ_LINE);
    if find_note(file_name.clone()) {
        fs::remove_file(&file_name).expect("Failed to delete file");
        println!("{file_name} successfully deleted.");
    } else {
        println!("{file_name} is not found.");
    }
}

fn get_all_notes() {
// TODO this function will use sqlite
    println!("get_all_notes fn");
}

fn find_note(note_name: String) -> bool {
    return Path::new(&note_name).exists();
}

fn create_new_note() {
    println!("Enter note name: ");
    let mut note_name = String::new();
    io::stdin().read_line(&mut note_name).expect(FAILED_TO_READ_LINE);
    if find_note(note_name.clone()) {
        println!("Note is exist. Just update it.");
    } else {
        let mut file = File::create(&note_name).expect("Failed to create file.");
        println!("Enter what you want write to note: ");
        let mut note_string = String::new();
        io::stdin().read_line(&mut note_string).expect("Failed to enter string line.");
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
