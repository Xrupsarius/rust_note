use std::io;
use std::process::exit;


fn main() {
    println!("This is note example app.");
    println!("Input your name: ");
    let mut input = String::new();
    // io::stdin().read_line(&mut input)?;
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    println!("Hello, {}", input);

    loop {
        main_menu();
        println!("Enter number: ");
        let mut menu_number = String::new();
        io::stdin().read_line(&mut menu_number).expect("Failed to read line.");
        let menu_num: i8 = menu_number.trim().parse().expect("This is not a number");
        select_menu_num(menu_num);
    }
}

fn select_menu_num(menu_num: i8) {
    if menu_num == 1 {
        create_new_note();
    } else if menu_num == 2 {
        find_note();
    } else if menu_num == 3 {
        get_all_notes();
    } else if menu_num == 4 {
        remove_note();
    } else if menu_num == 5 {
        println!("Have a nice day.");
        exit(0);
    }
}

fn remove_note() {
    println!("remove_note fn");
}

fn get_all_notes() {
    println!("get_all_notes fn");
}

fn find_note() {
    println!("Find note function.");
}

fn create_new_note() {
    println!("Create new note function.");
}

fn main_menu() {
    println!("Now input number what to do next: ");
    println!("[1] Create new note");
    println!("[2] Find note");
    println!("[3] Get all notes");
    println!("[4] Remove note");
    println!("[5] Exit");
}
