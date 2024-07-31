use std::i32;

use bracket_lib::prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Fill in this stub later
        self.mode = GameMode::End;
        println!("playing....");
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        println!("restart....");
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn main_play(&mut self, ctx: &mut BTerm) {}

    fn main_dead(&mut self, ctx: &mut BTerm) {}
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.main_play(ctx),
            GameMode::End => self.main_dead(ctx),
        }
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}
