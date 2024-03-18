use anyhow::{anyhow as error, Result};
use invader::{
    audio,
    container::{Container, Point},
    game::Game,
    game_buffer::GameBuffer,
    keyboard::KeyboardHandler,
    renderer::TerminalRenderer,
};

use crossterm::event::KeyCode;

fn main() -> Result<()> {
    let window_size = Container::new(Point { x: 0, y: 0 }, Point { x: 150, y: 35 });

    let tr = TerminalRenderer::new(window_size);
    tr.clear_screen()?;

    let mut game = Game::default();
    let mut gb = GameBuffer::new(&game);

    let mut key_handler = KeyboardHandler::new();

    loop {
        let key_event = key_handler.handle(&mut game);

        let keep_going = game.tick();

        gb.draw(&game);

        gb.draw_text(&game);
        tr.draw(&gb)?;

        if !keep_going {
            break;
        }

        if let Some(v) = key_event {
            if let KeyCode::Esc = v.code {
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    Ok(())
}
