use crossterm::{
    cursor, event::{poll, read, Event::Key, KeyCode}, style::{self, Stylize}, terminal::{self, BeginSynchronizedUpdate, Clear, EndSynchronizedUpdate}, ExecutableCommand, QueueableCommand
};
use std::{
    io,
    time::{Instant, Duration},
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let start = Instant::now();
    const FRAMETIME: f64 = 1./12.;
    terminal::enable_raw_mode()?;

    loop {
        stdout.execute(BeginSynchronizedUpdate)?;
        stdout.queue(Clear(terminal::ClearType::All))?;
        let (cols, rows) = terminal::size()?;
        for y in 0..rows {
            for x in 0..cols {
                // in this loop we are more efficient by not flushing the buffer.
                let intensity = (
	    			(10. * 
                        f64::sin(
	    				x as f64 * 0.1 
	    				- y as f64 * 0.15 
	    				+ start.elapsed().as_secs_f64() * 2.)
                    ) as i8
                    + 30
                ) as u8;

                let color = if y == 0 || y == rows - 1 || x == 0 || x == 1 || x == cols - 1 || x == cols - 2 {
                    style::Color::Rgb {
                        r: intensity + 30, 
                        g: 40,
                        b: intensity + 30,
                    }
                } else {
                    style::Color::Rgb {
                        r: intensity, 
                        g: 0,
                        b: intensity,
                    }
                };

                stdout
                    .queue(style::PrintStyledContent(" ".on(color)))?;
            }
        }
        stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.execute(EndSynchronizedUpdate)?;
        if poll(Duration::from_secs_f64(FRAMETIME - (start.elapsed().as_secs_f64() % FRAMETIME)))? {
            match read()? {
                Key(key) => if key.code == KeyCode::Char('q') { break; },
                _ => {}
            }
        }
    }
    stdout.execute(Clear(terminal::ClearType::All))?;
    terminal::disable_raw_mode()?;
    println!("bye bye");
    return Ok(());
}
