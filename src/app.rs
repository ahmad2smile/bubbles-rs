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

use crate::core::{lifecycle::LifeCycle, render::Render};

pub struct App {
    pub fps: u8,
}

impl App {
    pub fn new() -> Self {
        Self { fps: 60 }
    }

    pub fn set_fps(&mut self, fps: u8) -> &mut Self {
        self.fps = fps;

        self
    }

    pub fn run<C>(&self, root: &mut C) -> Result<()>
    where
        C: Render + LifeCycle,
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

    fn app_loop<C>(&self, root: &mut C, stdout: &mut Stdout) -> Result<()>
    where
        C: Render + LifeCycle,
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
        C: Render + LifeCycle,
    {
        let view = root.render();

        root.handle_render();

        let mut content = view.content.stylize().with(view.color);

        content.as_mut().background_color = Some(view.background);

        stdout
            .queue(cursor::MoveTo(view.dimension.x, view.dimension.y))?
            .queue(style::PrintStyledContent(content))?;
        stdout.flush()?;

        Ok(())
    }
}
