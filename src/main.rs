use crossterm::{
    cursor, 
    event::{poll, read, Event::Key, KeyCode},
    style::{self, Stylize},
    terminal::{self, BeginSynchronizedUpdate, Clear, EndSynchronizedUpdate}, 
    ExecutableCommand, QueueableCommand
};
use std::{
    io::{Result, Stdout, stdout},
    time::{Duration, Instant},
};
use std::vec;


struct App {
    stdout: Stdout,
    screen: Vec<Vec<char>>,
    cols: u16,
    rows: u16,
    cursor: (u16, u16),
    running: bool,
    start: Instant,
}

impl App {
    fn render(&mut self) -> Result<()>{
        self.stdout.execute(BeginSynchronizedUpdate)?;
        self.stdout.queue(Clear(terminal::ClearType::All))?;
        self.stdout.queue(cursor::MoveTo(0, 0))?;

        for y in 0..self.rows {
            for x in 0..self.cols {
                // in this loop we are more efficient by not flushing the buffer.
                let intensity = (
	    			(8. * 
                        f64::sin(
	    				x as f64 * 0.1 
	    				- y as f64 * 0.15 
	    				+ self.start.elapsed().as_secs_f64() * 3.)
                    ) as i8
                    + 30
                ) as u8;

                let color = if y == 0 || y == self.rows - 1 || x == 0 || x == 1 || x == self.cols - 1 || x == self.cols - 2 {
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

                self.stdout.queue(style::PrintStyledContent(self.screen[y as usize][x as usize].on(color)))?;
            }
        }
        self.stdout.queue(cursor::MoveTo(self.cursor.0, self.cursor.1))?;
        self.stdout.execute(EndSynchronizedUpdate)?;
        return Ok(());
    }

    fn handle_input(&mut self) -> Result<()> {
        if self.running {
            if poll(Duration::from_secs_f64(FRAMETIME - (self.start.elapsed().as_secs_f64() % FRAMETIME)))? {
                match read()? {
                    Key(key) => match key.code {
                        KeyCode::Char('q') => self.exit()?,
                        KeyCode::Right => {if self.cursor.0 < self.cols { self.cursor.0 += 1 } },
                        KeyCode::Left => {if self.cursor.0 > 0 { self.cursor.0 -= 1 } },
                        KeyCode::Down => {if self.cursor.1 < self.rows { self.cursor.1 += 1 } },
                        KeyCode::Up => {if self.cursor.1 > 0 { self.cursor.1 -= 1 } },
                        _ => {}
                    },
                    
                    _ => {}
                }
            }
        }
        return Ok(());
    }

    fn exit(&mut self) -> Result<()> {
        self.running = false;
        self.stdout.execute(Clear(terminal::ClearType::All))?;
        self.stdout.execute(cursor::MoveTo(0, 0))?;
        return Ok(());
    }

    fn run(&mut self) -> Result<()> {
        loop {
            self.render()?;
            self.handle_input()?;
            if !self.running { return Ok(()); }
        }
    }
}

const FRAMETIME: f64 = 1./12.;

fn main() -> Result<()> {
    let stdout = stdout();
    let start = Instant::now();
    terminal::enable_raw_mode()?;
    let cols = terminal::size()?.0;
    let rows = terminal::size()?.1;

    let mut app = App { 
        stdout,
        screen: vec![vec![' '; cols as usize]; rows as usize],
        cols,
        rows,
        cursor: (0, 0),
        running: true,
        start,
    };

    app.run()?;

    terminal::disable_raw_mode()?;
    println!("bye bye");
    return Ok(());
}
