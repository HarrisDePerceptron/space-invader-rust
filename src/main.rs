use anyhow::{anyhow as error, Result};
use invader::{
    audio,
    container::{Container, Point},
    game::{Game, GameCondition},
    game_buffer::GameBuffer,
    keyboard::KeyboardHandler,
    renderer::TerminalRenderer,
};

use crossterm::event::KeyCode;

struct GameManager {
    game: Game,
    tr: TerminalRenderer,
    key_handler: KeyboardHandler,
    gb: GameBuffer,

    raw_toogle: bool,
}

impl GameManager {
    pub fn new(width: usize, height: usize) -> Self {
        let mut game = Game::default();
        game.set_window(width, height);

        let mut tr = TerminalRenderer::new(&game);
        let mut gb = GameBuffer::new(&game);
        let mut key_handler = KeyboardHandler::new(&mut tr);

        Self {
            game,
            tr,
            key_handler,
            gb,
            raw_toogle: true,
        }
    }

    pub fn get_game(&mut self) -> &mut Game {
        &mut self.game
    }

    fn game_loop(&mut self) -> Result<GameCondition> {
        let mut game_condition = GameCondition::Ended;

        loop {
            let key_event = self.key_handler.handle(&mut self.game);

            game_condition = self.game.tick();

            self.gb.draw(&self.game);

            self.tr.draw(&self.gb)?;

            if let GameCondition::Win | GameCondition::Loss | GameCondition::Ended = game_condition
            {
                break;
            }

            if let Some(v) = key_event {
                if let KeyCode::Esc = v.code {
                    game_condition = GameCondition::Ended;

                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        Ok(game_condition)
    }

    pub fn main_loop(&mut self) -> Result<()> {
        let mut end_game = false;
        self.tr.clear_screen()?;

        let mut last_game = GameCondition::Ended;

        loop {
            self.tr.disable_cursor();
            self.tr.enable_raw_mode();

            last_game = self.game_loop()?;

            self.tr.disable_raw_mode();
            self.tr.enable_cursor();

            if let GameCondition::Ended = last_game {
                self.tr.draw_gameover1(&self.game, "Game Over");
            }

            if let GameCondition::Win = last_game {
                self.tr.draw_gameover1(&self.game, "You Win!!!");
            }

            if let GameCondition::Loss = last_game {
                self.tr.draw_gameover1(&self.game, "You Loose :(");
            }

            self.tr.disable_cursor();
            self.tr.enable_raw_mode();
            loop {
                let key_event = self.key_handler.handle(&mut self.game);
                if let Some(v) = key_event {
                    if let KeyCode::Esc = v.code {
                        end_game = true;
                        break;
                    } else if let KeyCode::Enter = v.code {
                        break;
                    }
                }
            }

            if end_game {
                break;
            }
            let max_lives = self.game.get_max_lives();
            let last_score = self.game.get_score();
            let last_lives = self.game.get_lives();

            self.game.reset_game();
            if let GameCondition::Win = last_game {
                self.game.set_lives(last_lives + 1);
                self.game.set_score(last_score);

                let (rows, cols) = self.game.get_enemies_rows_cols();
                let rows = rows + 1;
                let cols = cols + 1;

                self.game.set_enemy_rows_cols(rows, cols);
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        self.tr.disable_raw_mode();
        self.tr.enable_cursor();

        Ok(())
    }
}

fn main() -> Result<()> {
    let mut game_manager = GameManager::new(128, 32);
    let game = game_manager.get_game();

    game.set_enemy_rows_cols(5, 2);
    game_manager.main_loop()?;

    Ok(())
}
