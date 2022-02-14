use bracket_lib::prelude::*;

enum GameMode {
    SIMPLE,
    FANCY,
}

struct State {
    mode: GameMode
}

impl State {
    fn new() -> State {
        State {
            mode: GameMode::SIMPLE
        }
    }

    /*
        Press S to enable "SIMPLE" mode ("on" by default) and render "S".
        Press F to enable "FANCY" mode to render "F".
        I expect the code below to render either "F" or "S" but not both at the same time.
    */
    fn render(&self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::SIMPLE => {
                ctx.cls();
                ctx.set(10, 10, RED, BLACK, to_cp437('S'));

                ctx.set_active_console(1);
                ctx.cls();
                // uncomment below to "fix it" and clear existing symbols on fancy console by drawing a new one (huh?)
                // ctx.set(0, 0, RED, BLACK, to_cp437('E'));

                // try to guess what will happen if you uncomment both lines, above and below :)
                // ctx.cls();
                ctx.set_active_console(0);
            },
            GameMode::FANCY => {
                ctx.cls();

                ctx.set_active_console(1);
                ctx.cls();
                ctx.set(15, 10, RED, BLACK, to_cp437('F'));
                ctx.set_active_console(0);
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::S => {
                    self.mode = GameMode::SIMPLE;
                },
                VirtualKeyCode::F => {
                    self.mode = GameMode::FANCY;
                },
                _ => { }
            }
        }

        self.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_fancy_console(80, 50, "terminal8x8.png")
        .with_title("Reproduce fancy render issue")
        .build()?;
    main_loop(context, State::new())
}
