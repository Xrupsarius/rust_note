mod conn_configuration;
mod tui;

use conn_configuration as sql;
use std::{env, fs, io};
use std::fs::{File, OpenOptions};
use std::io::{Write};
use std::path::Path;
use std::process::exit;
use color_eyre::{
    eyre::{bail, WrapErr},
    Result
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Borders},
    Frame,
};

static FAILED_TO_READ_LINE: &'static str = "Failed to read line.";

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = tui::init()?;
    let result = App::default().run(&mut terminal);
    if let Err(err) = tui::restore() {
        eprintln!(
            "Failed to restore terminal. Run 'reset' or restart your terminal to recover: {}",
            err
        );
    }
    result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {

    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("key event failed:\n {key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter()?,
            KeyCode::Right => self.increment_counter()?,
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) -> Result<()> {
        self.counter += 1;
        Ok(())
    }

    fn decrement_counter(&mut self) -> Result<()> {
        self.counter -= 1;
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            " <Left>".blue().bold(),
            " Increment ".into(),
            " <Right>".blue().bold(),
            " Quit ".into(),
            " <Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

// fn run(mut terminal: DefaultTerminal) -> Result<()> {
//     loop {
//         terminal.draw(render)?;
//         if matches!(event::read()?, Event::Key(_)) {
//             break Ok(());
//         }
//     }
// }

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

// fn main() {
//     println!("This is note example app.");
//     println!("Input your name: ");
//     let input = read_input();
//     sql::sqlite_utils::create_database(input.clone());
//     println!("Hello, {input}");
//
//     loop {
//         main_menu();
//         println!("Enter number: ");
//         let menu_number = read_input();
//         let menu_num: i8 = menu_number.trim().parse().expect("This is not a number");
//         select_menu_num(menu_num);
//     }
// }

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
        println!("Note {note_name} does not exist.");
        loop {
            println!("Do you want to create a new note? (Y|N)");
            let choice = read_input().to_uppercase();

            match choice.as_str() {
                "Y" => {
                    File::create(note_name.clone()).unwrap();
                    break;
                }
                "N" => {
                    println!("Option cancelled");
                    return;
                }
                _ => println!("Invalid option. Please enter Y or N"),
            }
        }
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
