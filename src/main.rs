use crossterm::{
    cursor, 
    event::{poll, read, Event::Key, KeyCode},
    style::{self, Color, Stylize},
    terminal::{self, BeginSynchronizedUpdate, Clear, EndSynchronizedUpdate}, 
    ExecutableCommand, QueueableCommand
};
use chrono::{NaiveDate, Datelike, Weekday};
use std::{
    io::{stdout, Result, Stdout}, time::{Duration, Instant}, vec
};

const FRAMETIME: f64 = 1./12.;

const X_MAX: usize = 105;
const Y_MAX: usize = 36; 

#[derive(Clone, Copy)]
enum Subject {
    Film,
    Physics,
    Stats,
    APUSH,
    Compsci,
    Lang,
    None,
}

#[derive(Clone)]
struct Task {
    subject: Subject,
    description: String,
    date: chrono::NaiveDate,
    importance: usize,
}

struct App {
    stdout: Stdout,
    screen_text: Vec<Vec<char>>,
    screen_color: Vec<Vec<Color>>,
    running: bool,
    start: Instant,
    tab: usize,
    tasks: Vec<Task>,
    today: NaiveDate
}


fn main() -> Result<()> {
    let mut stdout = stdout();
    let tasks = vec![
        Task { 
            subject: Subject::APUSH,
            description: "example description".to_string(),
            date: NaiveDate::from_ymd_opt(2025, 9, 1).expect("That's not a valid date"),
            importance: 2
        }
    ];

    terminal::enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;

    let (cols, rows) = terminal::size()?;

    let mut app = App::new(stdout, cols as usize, rows as usize, tasks);

    app.run()?;

    terminal::disable_raw_mode()?;
    println!("bye bye");
    return Ok(());
}

impl App {
    fn new(stdout: Stdout, cols: usize, rows: usize, tasks: Vec<Task>) -> App {
        return App { 
            stdout,
            screen_text: vec![vec![' '; cols]; rows],
            screen_color: vec![vec![Color::White; cols]; rows],
            running: true,
            start: Instant::now(),
            tab: 0,
            tasks,
            today: chrono::Local::now().date_naive()
        };
    }

    fn run(&mut self) -> Result<()> {
        self.render_frame();
        self.render_tabs();

        self.render_string("▄▄              █  ▄▄   ▄▄▄     ▄▄     ", 60, 1);
        self.render_string("█ █ █▀█ █▄▀ █▀█   ▀▄     █  █▀█ █ █ █▀█", 60, 2);
        self.render_string("█▄▀ █▄█ █   █▄█▄  ▄▄▀    █  █▄█ █▄▀ █▄█", 60, 3);

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
            1 => self.render_calendar(),
            2 => {}, // self.render_something();
            _ => {}, // this will never happen
        }
        self.render_tabs();
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

    fn render_string(&mut self, string: &str, x: usize, y: usize) {
        let mut chars = string.chars();
        for i in 0..chars.clone().count() {
            self.screen_text[y][x + i] = chars.next().expect("what that wasnt supposed to happen");
        }
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

        self.render_string("Today", 8, 2);
        self.render_string("This Week", 22, 2);
        self.render_string("This Month", 37, 2);
    }

    fn render_tabs(&mut self) {
        for i in 3..=50 {
            self.screen_text[4][i] = '─';
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
                self.screen_text[4][2] = '╭';
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

    fn task_colors(&mut self) -> Vec<Color> {
        let mut colors: Vec<Color> = Vec::new();
        for task in self.tasks.clone() {
            let mut color = match task.subject {
                Subject::Film => Color::Rgb{ r: 127, g: 0, b: 0 },
                Subject::Physics => Color::Rgb{ r: 192, g: 127, b: 0 },
                Subject::Stats => Color::Rgb{ r: 0, g: 127, b: 127 },
                Subject::APUSH => Color::Rgb{ r: 0, g: 127, b: 0 },
                Subject::Compsci => Color::Rgb{ r: 0, g: 0, b: 127 },
                Subject::Lang => Color::Rgb{ r: 127, g: 0, b: 127 },
                Subject::None => Color::Grey,
            };
        }
        return colors;
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
        
        for i in 2..=7 {
            let row = HORIZONTAL_SPACING * (i) + 1;
            for j in 4..(X_MAX - 5) {
                self.screen_text[row][j] = '─';
            }
            self.screen_text[row][3] = '├';
            self.screen_text[row][X_MAX - 5] = '┤';
        }

        const WEEKDAY_HIGHLIGHT: Color = Color::Rgb{r: 255, g: 180, b: 255 }; 
        let y = match self.today.weekday(){
            Weekday::Sun => 5,
            Weekday::Mon => 9,
            Weekday::Tue => 13,
            Weekday::Wed => 17,
            Weekday::Thu => 21,
            Weekday::Fri => 25,
            Weekday::Sat => 29,
        };
        self.color_area(WEEKDAY_HIGHLIGHT, 4, y, X_MAX - 6, y);
        self.color_area(WEEKDAY_HIGHLIGHT, 4, y + 4, X_MAX - 6, y + 4);

        for task in self.tasks.clone() {
            if task.date.week(Weekday::Sun) == self.today.week(Weekday::Sun) {
                let y = match task.date.weekday() {
                    Weekday::Sun => 6,
                    Weekday::Mon => 10,
                    Weekday::Tue => 14,
                    Weekday::Wed => 18,
                    Weekday::Thu => 22,
                    Weekday::Fri => 26,
                    Weekday::Sat => 30,
                };
            }
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
        self.draw()?;

        return Ok(());
    }
}
