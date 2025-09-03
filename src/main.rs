use crossterm::{
    cursor, 
    event::{poll, read, Event::Key, KeyCode, Event::Resize},
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
    width: usize,
    height: usize,
    today: NaiveDate
}


fn main() -> Result<()> {
    let mut stdout = stdout();
    let tasks = vec![
        Task::new(Subject::Film, "Finish PSA Script", 2025, 9, 3),
        Task::new(Subject::Stats, "Homework 2", 2025, 9, 4),
        Task::new(Subject::APUSH, "AMSCO MCQs + SAQ Pg. 22", 2025, 9, 4),
    ];
    let (width, height) = terminal::size()?;

    stdout.execute(terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::DisableLineWrap)?;

    let mut app = App::new(stdout, tasks, width as usize, height as usize);
    // app.today = NaiveDate::from_ymd_opt(2025, 10, 1).unwrap();
    app.run()?;

    terminal::disable_raw_mode()?;
    println!("bye bye");
    return Ok(());
}

impl App {
    fn new(stdout: Stdout, tasks: Vec<Task>, width: usize, height: usize) -> App {
        return App { 
            stdout,
            screen_text: vec![vec![' '; width]; height],
            screen_color: vec![vec![Color::White; width]; height],
            running: true,
            start: Instant::now(),
            tab: 0,
            tasks,
            width,
            height,
            today: chrono::Local::now().date_naive()
        };
    }

    fn run(&mut self) -> Result<()> {
        self.render_frame();
        self.render_tabs();

        while self.running {
            self.draw()?;
            self.handle_input()?;
        }
        return Ok(());
    }

    fn exit(&mut self) -> Result<()> {
        self.running = false;
        self.stdout.execute(Clear(terminal::ClearType::All))?;
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        return Ok(());
    }

    fn clear_tab(&mut self) {
        for i in 1..(self.width - 2) {
            for j in 4..(self.height - 1) {
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
                        KeyCode::Char('q') | KeyCode::Char('Q') => self.exit()?,
                        KeyCode::Tab => self.switch_tab(false),
                        KeyCode::BackTab => self.switch_tab(true),
                        // KeyCode::Right => if self.cursor.0 < self.width { self.cursor.0 += 1 },
                        // KeyCode::Left => if self.cursor.0 > 0 { self.cursor.0 -= 1 },
                        // KeyCode::Down => if self.cursor.1 < self.height { self.cursor.1 += 1 },
                        // KeyCode::Up => if self.cursor.1 > 0 { self.cursor.1 -= 1 },
                        _ => {}
                    },

                    Resize(width, height) => self.resize(width as usize, height as usize),
                    
                    _ => {}
                }
            }
        }
        return Ok(());
    }

    fn resize(&mut self, new_width: usize, new_height: usize) {
        if new_height > self.height {
            self.screen_text.resize(new_height, vec![' '; new_width]);
        }
        if new_width > self.width {
            for row in &mut self.screen_text {
                row.resize(new_width, ' ');
            }
        }

        self.render_frame();
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
        self.screen_text[0][0] = '╭';
        self.screen_text[1][0] = '│';
        self.screen_text[2][0] = '│';

        self.screen_text[1][12] = '│';
        self.screen_text[2][12] = '│';

        self.screen_text[1][24] = '│';
        self.screen_text[2][24] = '│';

        self.screen_text[1][37] = '│';
        self.screen_text[2][37] = '│';

        self.screen_text[0][50] = '╮';
        self.screen_text[1][50] = '│';
        self.screen_text[2][50] = '│';

        self.screen_text[3][0] = '╭';
        self.screen_text[3][self.width - 2] = '╮';
        self.screen_text[self.height - 1][0] = '╰';
        self.screen_text[self.height - 1][self.width - 2] = '╯';

        for i in 1..(self.width - 2) {
            self.screen_text[3][i] = '─';
            self.screen_text[self.height - 1][i] = '─';
        }

        for i in 4..(self.height - 1) {
            self.screen_text[i][0] = '│';
            self.screen_text[i][self.width - 2] = '│';
        }

        self.render_string("Today", 4, 1);
        self.render_string("This Week", 14, 1);
        self.render_string("This Month", 26, 1);
        self.render_string("Add Task", 40, 1);

        if self.width > 90 {
            self.render_string("▄▄              ▄  ▄▄   ▄▄▄     ▄▄     ", self.width - 40, 0);
            self.render_string("█ █ █▀█ █▄▀ ▄▀█ ▀ ▀▄     █  █▀█ █ █ █▀█", self.width - 40, 1);
            self.render_string("█▄▀ █▄█ █   ▀▄█   ▄▄▀    █  █▄█ █▄▀ █▄█", self.width - 40, 2);
        }
    }

    fn render_tabs(&mut self) {
        for i in 1..50 {
            self.screen_text[0][i] = '─';
            self.screen_text[3][i] = '─';
        }
        self.screen_text[3][50] = '─';

        self.screen_text[3][0] = '╭';
        self.screen_text[0][12] = '┬';
        self.screen_text[0][24] = '┬';
        self.screen_text[0][37] = '┬';

        self.color_area(Color::DarkGrey, 0, 0, 50, 2);
        match self.tab {
            0 => {
                self.color_area(Color::White, 0, 0, 12, 2);
                self.screen_text[0][12] = '╮';
                for i in 1..12 {
                    self.screen_text[3][i] = ' ';
                }
                self.screen_text[3][0] = '│';
                self.screen_text[3][12] = '╰';
            },
            1 => { 
                self.color_area(Color::White, 12, 0, 24, 2);
                self.screen_text[0][12] = '╭';
                self.screen_text[0][24] = '╮';
                self.screen_text[3][12] = '╯';
                self.screen_text[3][24] = '╰';
                for i in 13..24 {
                    self.screen_text[3][i] = ' ';
                }
            },
            2 => { 
                self.color_area(Color::White, 24, 0, 37, 2);
                self.screen_text[0][24] = '╭';
                self.screen_text[0][37] = '╮';
                self.screen_text[3][24] = '╯';
                self.screen_text[3][37] = '╰';
                for i in 25..37 {
                    self.screen_text[3][i] = ' ';
                }
            },
            3 => { 
                self.color_area(Color::White, 37, 0, 50, 2);
                self.screen_text[0][37] = '╭';
                self.screen_text[3][37] = '╯';
                self.screen_text[3][50] = '╰';
                for i in 38..50 {
                    self.screen_text[3][i] = ' ';
                }
            },
            _ => {}
        }
    }

    fn draw(&mut self) -> Result<()> {
        self.stdout.execute(BeginSynchronizedUpdate)?;
        self.stdout.queue(Clear(terminal::ClearType::Purge))?;
        self.stdout.queue(cursor::MoveTo(0, 0))?;

        for y in 0..self.height {
            for x in 0..self.width {
                let intensity = (
	    			(8. * 
                        f64::sin(
	    				x as f64 * 0.1 
	    				- y as f64 * 0.15 
	    				+ self.start.elapsed().as_secs_f64() * 3.)
                    ) as i8
                    + 30
                ) as u8;

                self.stdout.queue(style::PrintStyledContent(
                    self.screen_text[y][x]
                    .with(self.screen_color[y][x])
                    .on(style::Color::Rgb{r: intensity, g: 0, b: intensity})
                ))?;
            }
            self.stdout.execute(cursor::MoveToNextLine(1))?;
        }
        self.stdout.execute(EndSynchronizedUpdate)?;
        return Ok(());
    }
}
