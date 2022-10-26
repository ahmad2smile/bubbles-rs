use crossterm::{
    cursor, event,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{self, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
    QueueableCommand, Result,
};
use std::{
    io::{stdout as std_init, Stdout, Write},
    time::{Duration, Instant},
};

use crate::core::component::Component;

pub struct App {
    pub fps: u8,
    components: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            fps: 60,
            components: vec![],
        }
    }

    pub fn set_fps(&mut self, fps: u8) -> &mut Self {
        self.fps = fps;

        self
    }

    pub fn run(&mut self) -> Result<()> {
        let mut stdout = std_init();

        enable_raw_mode()?;

        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout, LeaveAlternateScreen, EnableMouseCapture)?;

        self.app_loop(&mut stdout)?;

        disable_raw_mode()?;
        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }

    fn app_loop(&mut self, stdout: &mut Stdout) -> Result<()> {
        let mut tick = Instant::now();
        let tick_rate = Duration::from_millis((1 / self.fps).into());

        loop {
            self.render(stdout, None)?;

            // Delay b/w frames
            let timeout = tick_rate
                .checked_sub(tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                match event::read()? {
                    event => match event {
                        Event::Key(KeyEvent {
                            modifiers: KeyModifiers::CONTROL,
                            code: KeyCode::Char('c'),
                            ..
                        }) => return Ok(()),
                        Event::Key(event) => {
                            self.render(stdout, Some(event))?;
                        }
                        // Event::FocusGained => todo!(),
                        // Event::FocusLost => todo!(),
                        // Event::Mouse(_) => todo!(),
                        // Event::Paste(_) => todo!(),
                        // Event::Resize(_, _) => todo!(),
                        _ => (),
                    },
                };
            }

            if tick.elapsed() >= tick_rate {
                tick = Instant::now();
            }
        }
    }

    fn render(&mut self, stdout: &mut Stdout, key_event: Option<KeyEvent>) -> Result<()> {
        for component in &mut self.components {
            component.handle_render();

            if let Some(key_e) = key_event {
                component.handle_key_event(key_e);
            }

            let view = component.render();

            let mut content = view.content.stylize().with(view.color);

            content.as_mut().background_color = Some(view.background);

            stdout
                .queue(cursor::MoveTo(view.dimension.x, view.dimension.y))?
                .queue(style::PrintStyledContent(content))?;
        }

        stdout.flush()?;

        Ok(())
    }

    pub fn register(&mut self, c: Box<dyn Component>) {
        self.components.push(c);
    }
}
