use std::error::Error;
use std::io;

use crossterm::{event, terminal, ExecutableCommand};

fn main() -> Result<(), Box<dyn >> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");
    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide);

    // game loop
    'gameloop: loop {
        // input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => menu.change_option(true),
                    KeyCode::Down => menu.change_option(false),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if menu.selection == 0 {
                            in_menu = false;
                        } else {
                            break 'gameloop;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
