use std::{
    io::{stdout, Write},
    process, thread,
};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEventKind},
    queue,
    style::{self, Stylize},
    terminal::{self},
    ExecutableCommand, QueueableCommand,
};

use arboard::Clipboard;

use liblbx::cli::{
    events::EventHandler,
    io::master,
    ui::menu::{Item, Menu},
};

fn main() {
    let master_pass = master::get_master_password().unwrap_or_else(|err| {
        eprintln!("An error occured: {err}");
        process::exit(1)
    });

    terminal::disable_raw_mode().unwrap();

    let mut stdout = stdout();

    let (cols, rows) = terminal::size().unwrap();
    stdout
        .queue(terminal::ScrollDown(rows))
        .unwrap()
        .queue(cursor::Hide)
        .unwrap();
    stdout.queue(cursor::MoveTo(1, 1)).unwrap();

    println!("the master pass is: {master_pass:?}");
    stdout.flush().unwrap();

    let (event_handler, event_rx, kill_tx) = EventHandler::new();

    let event_thread = event_handler.spawn();

    let mut main_menu = Menu::builder()
        .items(vec![Item::new("Passwords"), Item::new("Exit")])
        .offset_top(3)
        .padding(3)
        .build();

    main_menu.display(&mut stdout).unwrap();

    while let Ok(e) = event_rx.recv() {
        match e.code {
            KeyCode::Up => main_menu.prev_item(&mut stdout).unwrap(),
            KeyCode::Down => main_menu.next_item(&mut stdout).unwrap(),
            KeyCode::Enter => match *main_menu.get_current_item().get_raw_item() {
                "Exit" => {
                    kill_tx.send(()).unwrap();
                }
                "Passwords" => {}
                _ => {}
            },
            _ => {}
        }
    }

    event_thread.join().unwrap();

    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::FromCursorDown),
        cursor::Show
    )
    .unwrap();
    stdout.flush().unwrap();
}
