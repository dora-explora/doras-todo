use crossterm::{
    cursor, 
    event::{poll, read, Event::Key, KeyCode},
    style::{self, Color, Stylize},
    terminal::{self, BeginSynchronizedUpdate, Clear, EndSynchronizedUpdate}, 
    ExecutableCommand, QueueableCommand
};
use chrono::NaiveDate;
use std::{
    io::{stdout, Result, Stdout}, time::{Duration, Instant}, vec
};

mod tabs;

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
}

impl Task {
    fn new(subject: Subject, description: &str, year: i32, month: u32, day: u32) -> Task {
        return Task { 
            subject,
            description: description.to_string(),
            date: NaiveDate::from_ymd_opt(year, month, day).expect(format!("{month}/{day}/{year} is not a valid date").as_str()),
        };
    }
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
        Task::new(Subject::APUSH, "Unit 1 Chapter 1 Key Terms", 2025, 8, 24),
        Task::new(Subject::Lang, "Pages 4-6 & 10-12 Notes", 2025, 8, 26),
        Task::new(Subject::Physics, "Helioseismology Dimensional Analysis", 2025, 8, 27),
        Task::new(Subject::Stats, "Chapter 1 HW 1", 2025, 8, 28),
        Task::new(Subject::None, "Shabbat", 2025, 8, 29),
        Task::new(Subject::Compsci, "Credit Card Verifier", 2025, 8, 29),
        Task::new(Subject::Film, "Finish PSA Script", 2025, 8, 30),
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

    fn clear_tab(&mut self) {
        for i in 3..(X_MAX - 4) {
            for j in 5..(Y_MAX - 2) {
                self.screen_text[j][i] = ' ';
                self.screen_color[j][i] = Color::White;
            }
        }
    }

    fn switch_tab(&mut self, backward: bool) {
        self.tab += match backward { false => 1, true => 3 };
        self.tab %= 4;

        self.clear_tab();
        match self.tab {
            0 => self.render_today_tab(),
            1 => self.render_week_tab(),
            2 => self.render_month_tab(),
            3 => self.render_entry_tab(),
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
                        KeyCode::Tab => self.switch_tab(false),
                        KeyCode::BackTab => self.switch_tab(true),
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

        self.screen_text[2][14] = '│';
        self.screen_text[3][14] = '│';

        self.screen_text[2][26] = '│';
        self.screen_text[3][26] = '│';

        self.screen_text[2][39] = '│';
        self.screen_text[3][39] = '│';

        self.screen_text[1][52] = '╮';
        self.screen_text[2][52] = '│';
        self.screen_text[3][52] = '│';

        self.screen_text[4][2] = '╭';
        self.screen_text[4][X_MAX - 4] = '╮';
        self.screen_text[Y_MAX - 2][2] = '╰';
        self.screen_text[Y_MAX - 2][X_MAX - 4] = '╯';

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

        self.render_string("Today", 6, 2);
        self.render_string("This Week", 16, 2);
        self.render_string("This Month", 28, 2);
        self.render_string("Add Task", 42, 2);

        self.render_string("▄▄              ▄  ▄▄   ▄▄▄     ▄▄     ",59, 1);
        self.render_string("█ █ █▀█ █▄▀ ▄▀█ ▀ ▀▄     █  █▀█ █ █ █▀█",59, 2);
        self.render_string("█▄▀ █▄█ █   ▀▄█   ▄▄▀    █  █▄█ █▄▀ █▄█",59, 3);
    }

    fn render_tabs(&mut self) {
        for i in 3..52 {
            self.screen_text[1][i] = '─';
            self.screen_text[4][i] = '─';
        }

        self.screen_text[4][2] = '╭';
        self.screen_text[1][14] = '┬';
        self.screen_text[1][26] = '┬';
        self.screen_text[1][39] = '┬';

        self.color_area(Color::DarkGrey, 2, 1, 52, 3);
        match self.tab {
            0 => {
                self.color_area(Color::White, 2, 1, 14, 3);
                self.screen_text[1][14] = '╮';
                for i in 3..14 {
                    self.screen_text[4][i] = ' ';
                }
                self.screen_text[4][2] = '│';
                self.screen_text[4][14] = '╰';
                self.screen_text[4][52] = '─';
            },
            1 => { 
                self.color_area(Color::White, 14, 1, 26, 3);
                self.screen_text[1][14] = '╭';
                self.screen_text[1][26] = '╮';
                self.screen_text[4][14] = '╯';
                self.screen_text[4][26] = '╰';
                for i in 15..26 {
                    self.screen_text[4][i] = ' ';
                }
            },
            2 => { 
                self.color_area(Color::White, 26, 1, 39, 3);
                self.screen_text[1][26] = '╭';
                self.screen_text[1][39] = '╮';
                self.screen_text[4][26] = '╯';
                self.screen_text[4][39] = '╰';
                for i in 27..39 {
                    self.screen_text[4][i] = ' ';
                }
            },
            3 => { 
                self.color_area(Color::White, 39, 1, 52, 3);
                self.screen_text[1][39] = '╭';
                self.screen_text[4][39] = '╯';
                self.screen_text[4][52] = '╰';
                for i in 40..52 {
                    self.screen_text[4][i] = ' ';
                }
            },
            _ => {}
        }
    }

    fn draw(&mut self) -> Result<()> {
        self.stdout.execute(BeginSynchronizedUpdate)?;
        self.stdout.queue(Clear(terminal::ClearType::All))?;
        self.stdout.queue(cursor::MoveTo(0, 0))?;

        for y in 0..Y_MAX {
            for x in 0..X_MAX {
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
