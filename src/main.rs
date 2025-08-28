use crossterm::{
    cursor, 
    event::{poll, read, Event::Key, KeyCode},
    style::{self, Color, Stylize},
    terminal::{self, BeginSynchronizedUpdate, Clear, EndSynchronizedUpdate}, 
    ExecutableCommand, QueueableCommand
};
use std::{
    io::{Result, Stdout, stdout},
    time::{Duration, Instant},
    vec,
};

const FRAMETIME: f64 = 1./12.;

const X_MAX: usize = 105;
const Y_MAX: usize = 36; 

struct App {
    stdout: Stdout,
    screen_text: Vec<Vec<char>>,
    screen_color: Vec<Vec<Color>>,
    running: bool,
    start: Instant,
    tab: usize,
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    let start = Instant::now();

    terminal::enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;

    let (cols, rows) = terminal::size()?;
    let mut app = App { 
        stdout,
        screen_text: vec![vec![' '; cols as usize]; rows as usize],
        screen_color: vec![vec![Color::White; cols as usize]; rows as usize],
        running: true,
        start,
        tab: 0,
    };

    app.run()?;

    terminal::disable_raw_mode()?;
    println!("bye bye");
    return Ok(());
}

impl App {
    fn run(&mut self) -> Result<()> {
        while self.running {
            self.render()?;
            self.handle_input()?;
        }
        return Ok(());
    }

    fn exit(&mut self) -> Result<()> {
        self.running = false;
        self.stdout.execute(Clear(terminal::ClearType::All))?;
        self.stdout.execute(cursor::MoveTo(0, 0))?;
        return Ok(());
    }

    fn clear_view(&mut self) {
        for i in 3..(X_MAX - 4) {
            for j in 5..(Y_MAX - 2) {
                self.screen_text[j][i] = ' ';
                self.screen_color[j][i] = Color::White;
            }
        }
    }

    fn switch_tab(&mut self) {
        self.tab += 1;
        self.tab %= 3;

        self.clear_view();
        match self.tab {
            0 => {}, // self.render_something();
            1 => {}, // self.render_something();
            2 => self.render_calendar(),
            _ => {}, // this will never happen
        }
    }

    fn handle_input(&mut self) -> Result<()> {
        if self.running {
            if poll(Duration::from_secs_f64(FRAMETIME - (self.start.elapsed().as_secs_f64() % FRAMETIME)))? {
                match read()? {
                    Key(key) => match key.code {
                        KeyCode::Char('q') => self.exit()?,
                        KeyCode::Tab => self.switch_tab(),
                        // KeyCode::Right => if self.cursor.0 < X_MAX { self.cursor.0 += 1 },
                        // KeyCode::Left => if self.cursor.0 > 0 { self.cursor.0 -= 1 },
                        // KeyCode::Down => if self.cursor.1 < Y_MAX { self.cursor.1 += 1 },
                        // KeyCode::Up => if self.cursor.1 > 0 { self.cursor.1 -= 1 },
                        _ => {}
                    },
                    
                    _ => {}
                }
            }
        }
        return Ok(());
    }

    fn color_area(&mut self, color: Color, x_min: usize, y_min: usize, x_max: usize, y_max: usize) {
        for i in x_min..=x_max {
            for j in y_min..=y_max {
                self.screen_color[j][i] = color;
            }
        }
    }

    fn render_frame(&mut self) {
        self.screen_text[1][2] = '╭';
        self.screen_text[2][2] = '│';
        self.screen_text[3][2] = '│';

        self.screen_text[2][18] = '│';
        self.screen_text[3][18] = '│';

        self.screen_text[2][34] = '│';
        self.screen_text[3][34] = '│';

        self.screen_text[1][50] = '╮';
        self.screen_text[2][50] = '│';
        self.screen_text[3][50] = '│';

        self.screen_text[4][2] = '╭';
        self.screen_text[4][X_MAX - 4] = '╮';
        self.screen_text[Y_MAX - 2][2] = '╰';
        self.screen_text[Y_MAX - 2][X_MAX - 4] = '╯';

        for i in 3..50 {
            self.screen_text[1][i] = '─';
        }

        for i in 3..(X_MAX - 4) {
            self.screen_text[Y_MAX - 2][i] = '─';
            self.screen_text[4][i] = '─';
        }

        for i in 5..(Y_MAX - 2) {
            self.screen_text[i][2] = '│';
        }

        for i in 5..(Y_MAX - 2) {
            self.screen_text[i][X_MAX - 4] = '│';
        }

        self.color_area(Color::DarkGrey, 2, 1, 50, 3);

        match self.tab {
            0 => {
                self.color_area(Color::White, 2, 1, 18, 3);
                self.screen_text[1][18] = '╮';
                self.screen_text[1][34] = '┬';
                for i in 3..18 {
                    self.screen_text[4][i] = ' ';
                }
                self.screen_text[4][2] = '│';
                self.screen_text[4][18] = '╰';
            },
            1 => { 
                self.color_area(Color::White, 18, 1, 34, 3);
                self.screen_text[1][18] = '╭';
                self.screen_text[1][34] = '╮';
                for i in 19..34 {
                    self.screen_text[4][i] = ' ';
                }
                self.screen_text[4][18] = '╯';
                self.screen_text[4][34] = '╰';
            },
            2 => { 
                self.color_area(Color::White, 34, 1, 50, 3);
                self.screen_text[1][18] = '┬';
                self.screen_text[1][34] = '╭';
                for i in 35..50 {
                    self.screen_text[4][i] = ' ';
                }
                self.screen_text[4][34] = '╯';
                self.screen_text[4][50] = '╰';
            },
            _ => {}
        }
    }

    fn render_calendar(&mut self) {
        self.screen_text[5][3] = '┌';
        self.screen_text[5][X_MAX - 5] = '┐';
        self.screen_text[Y_MAX - 3][3] = '└';
        self.screen_text[Y_MAX - 3][X_MAX - 5] = '┘';
        
        for i in 4..(X_MAX - 5) {
            self.screen_text[5][i] = '─';
            self.screen_text[Y_MAX - 3][i] = '─';
        }

        for i in 6..(Y_MAX - 3) {
            self.screen_text[i][3] = '│';
            self.screen_text[i][X_MAX - 5] = '│';
        }

        const HORIZONTAL_SPACING: usize = 4;
        
        for i in 1..7 {
            let row = HORIZONTAL_SPACING * (i + 1) + 1;
            for j in 4..(X_MAX - 5) {
                self.screen_text[row][j] = '─';
            }
            self.screen_text[row][3] = '├';
            self.screen_text[row][X_MAX - 5] = '┤';
        }
    }

    fn draw(&mut self) -> Result<()> {
        self.stdout.execute(BeginSynchronizedUpdate)?;
        self.stdout.queue(Clear(terminal::ClearType::All))?;
        self.stdout.queue(cursor::MoveTo(0, 0))?;

        for y in 0..Y_MAX {
            for x in 0..X_MAX {
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

                let color = if y == 0 || y == Y_MAX - 1 || x == 0 || x == 1 || x == X_MAX - 1 || x == X_MAX - 2 {
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

                self.stdout.queue(style::PrintStyledContent(
                    self.screen_text[y as usize][x as usize]
                    .with(self.screen_color[y as usize][x as usize])
                    .on(color)
                ))?;
            }
        }
        self.stdout.execute(EndSynchronizedUpdate)?;
        return Ok(());
    }

    fn render(&mut self) -> Result<()> {
        self.render_frame();
        match self.tab {
            0 => {},
            1 => {},
            2 => self.render_calendar(),
            _ => {}
        }
        self.draw()?;

        return Ok(());
    }
}
