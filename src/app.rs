use crossterm::{
    cursor, event,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    style::{self, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
    QueueableCommand, Result,
};
use std::{
    io::{stdout as std_init, Stdout, Write},
    time::{Duration, Instant},
};

use crate::core::{render::Render, state::State};

pub struct App<'a> {
    pub fps: u8,
    update_handler: &'a dyn FnOnce(),
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            fps: 60,
            update_handler: &|| {},
        }
    }

    pub fn set_fps(&mut self, fps: u8) -> &mut Self {
        self.fps = fps;

        self
    }

    pub fn run<C>(&self, root: &mut C) -> Result<()>
    where
        C: Render + State<'a>,
    {
        let mut stdout = std_init();

        enable_raw_mode()?;

        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout, LeaveAlternateScreen, EnableMouseCapture)?;

        self.app_loop(root, &mut stdout)?;

        disable_raw_mode()?;
        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }

    pub fn on_update(&mut self, handler: &'a dyn FnOnce()) {
        self.update_handler = handler;
    }

    fn app_loop<C>(&self, root: &mut C, stdout: &mut Stdout) -> Result<()>
    where
        C: Render + State<'a>,
    {
        let mut tick = Instant::now();
        let tick_rate = Duration::from_millis((1 / self.fps).into());

        loop {
            self.render(root, stdout)?;

            // Delay b/w frames
            let timeout = tick_rate
                .checked_sub(tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
            }

            if tick.elapsed() >= tick_rate {
                tick = Instant::now();
            }
        }
    }

    fn render<C>(&self, root: &mut C, stdout: &mut Stdout) -> Result<()>
    where
        C: Render + State<'a>,
    {
        let view = root.render();

        root.handle_update();

        let mut content = view.content.stylize().with(view.color);

        content.as_mut().background_color = Some(view.background);

        stdout
            .queue(cursor::MoveTo(view.x, view.y))?
            .queue(style::PrintStyledContent(content))?;
        stdout.flush()?;

        Ok(())
    }
}
