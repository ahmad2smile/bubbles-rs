use crossterm::{
    cursor,
    style::{self, Color, Stylize},
    QueueableCommand, Result,
};
use std::io::{stdout as std_init, Write};

use crate::components::{Component, Instruction};

pub fn run(root: &dyn Component) -> Result<()> {
    let mut instructions = Vec::new() as Vec<Instruction>;

    root.render(&mut instructions);

    let mut stdout = std_init();

    // execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    for i in instructions {
        let mut content = i.content.stylize().with(i.color);

        content.as_mut().background_color = Some(Color::try_from(i.background).unwrap());

        stdout
            .queue(cursor::MoveTo(i.x, i.y))?
            .queue(style::PrintStyledContent(content))?;
    }

    stdout.flush()?;

    Ok(())
}
