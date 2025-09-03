use crossterm::style::Color;
use chrono::{Datelike, Days, Weekday};
use crate::{App, Task, Subject};

fn task_color(task: &Task) -> Color {
    let color: (u8, u8, u8) = match task.subject {
        Subject::Film => (255, 127, 127),
        Subject::Physics => (0, 255, 0),
        Subject::Stats => (0, 255, 255),
        Subject::APUSH => (255, 0, 0),
        Subject::Compsci => (0, 0, 255),
        Subject::Lang => (255, 255, 0),
        Subject::None => (127, 127, 127),
    };

    return Color::Rgb{ r: color.0, g: color.1, b: color.2 }; 
}

impl App {

    pub fn render_today_tab(&mut self) {

    }

    pub fn render_week_tab(&mut self) {
        let vertical_spacing: usize = (self.height - 5) / 7;
        
        for i in 0..7 {
            let row = vertical_spacing * (i) + 4;
            for j in 9..(self.width - 3) {
                self.screen_text[row][j] = '─';
            }
            self.screen_text[row][self.width - 3] = '┤';
        }

        self.render_string(" Sunday ",    1, 4);
        self.render_string(" Monday ",    1, vertical_spacing * 1 + 4);
        self.render_string(" Tuesday ",   1, vertical_spacing * 2 + 4);
        self.render_string(" Wednesday ", 1, vertical_spacing * 3 + 4);
        self.render_string(" Thursday ",  1, vertical_spacing * 4 + 4);
        self.render_string(" Friday ",    1, vertical_spacing * 5 + 4);
        self.render_string(" Saturday ",  1, vertical_spacing * 6 + 4);

        let (weekday, width) = match self.today.weekday(){
            Weekday::Sun => (0, 8),
            Weekday::Mon => (1, 8),
            Weekday::Tue => (2, 8),
            Weekday::Wed => (3, 11),
            Weekday::Thu => (4, 10),
            Weekday::Fri => (5, 8),
            Weekday::Sat => (6, 10),
        };
        let y = weekday * vertical_spacing + 4;
        self.color_area(Color::Rgb{r: 255, g: 200, b: 50 }, 1 + width, y, self.width - 3, y);

        let mut tasks_by_weekday: [Vec<Task>; 7] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for task in &self.tasks {
            if task.date.week(Weekday::Sun) == self.today.week(Weekday::Sun) {
                let task_weekday = match task.date.weekday(){
                    Weekday::Sun => 0,
                    Weekday::Mon => 1,
                    Weekday::Tue => 2,
                    Weekday::Wed => 3,
                    Weekday::Thu => 4,
                    Weekday::Fri => 5,
                    Weekday::Sat => 6,
                };
                tasks_by_weekday[task_weekday].push(task.clone());
            }
        }

        for weekday in 0..7 {
            let y = weekday * vertical_spacing + 4 + (vertical_spacing / 2);
            let tasks = &tasks_by_weekday[weekday];
            if tasks.len() == 1 {
                self.render_string("██ ", 2, y);
                self.render_string(&tasks[0].description, 5, y);
                self.color_area(task_color(&tasks[0]), 2, y, 3, y);  
            } else if tasks.len() > 1 {
                let mut width = 0;
                for i in 0..tasks.len() {
                    width += 6;
                    width += tasks[i].description.len();
                }
                width -= 3;
                let mut minimized = 0;
                while width > (self.width - 3) {
                    width = 0;
                    for i in 0..tasks.len() {
                        width += 5;
                        if i < (tasks.len() - minimized) {
                            width += tasks[i].description.len() + 1;
                        }
                    }
                    minimized += 1;
                }
                if minimized > 0 { minimized -= 1; }
                let mut x = 2;
                for i in 0..tasks.len() {
                    self.screen_text[y][x - 2] = '│';
                    self.render_string("██ ", x, y);
                    self.color_area(task_color(&tasks[i]), x, y, x + 1, y);
                    x += 5;
                    
                    if i < (tasks.len() - minimized) {
                        self.render_string(&tasks[i].description, x - 2, y);
                        x += tasks[i].description.len() + 1;
                    }
                }
            }
        }
    }

    pub fn render_month_tab(&mut self) {
        let horizontal_spacing = (self.width - 4) / 7;
        let vertical_spacing = (self.height - 6) / 6;
        let right = horizontal_spacing * 7 + 1;
        let bottom = vertical_spacing * 6 + 4;

        self.screen_text[4][1] = '┌';
        self.screen_text[4][right] = '┐';
        self.screen_text[bottom][1] = '└';
        self.screen_text[bottom][right] = '┘';

        for i in 2..right {
            self.screen_text[4][i] = '─';
            self.screen_text[bottom][i] = '─';
        }

        for i in 5..bottom {
            self.screen_text[i][1] = '│';
            self.screen_text[i][right] = '│';
        }

        for i in 1..7 {
            let column = horizontal_spacing * i + 1;
            self.screen_text[4][column] = '┬';
            for j in 5..bottom {
                self.screen_text[j][column] = '│';
            }
            self.screen_text[bottom][column] = '┴';
        }

        for i in 1..6 {
            let row = vertical_spacing * i + 4;
            self.screen_text[row][1] = '├';
            for j in 2..right {
                if j % horizontal_spacing == 1 {
                    self.screen_text[row][j] = '┼';
                } else {
                    self.screen_text[row][j] = '─';
                }
            }
            self.screen_text[row][right] = '┤';
        }

        let mut min_week = self.today.week(Weekday::Sun);
        while min_week.last_day().month() == self.today.month() {
            min_week = min_week.first_day().checked_sub_days(Days::new(7)).expect("date error???").week(Weekday::Sun); // this is just min_week--;
        }
        min_week = min_week.first_day().checked_add_days(Days::new(7)).expect("date error???").week(Weekday::Sun); // min_week++;
        let max_week = min_week.first_day().checked_add_days(Days::new(35)).expect("date error???").week(Weekday::Sun); // max_week = min_week + 5;
        for task in &self.tasks {
            if task.date >= min_week.first_day() && task.date <= max_week.last_day() {
                
            }
        }

        let mut first_day = min_week.first_day();
        let mut day_offset = 0;
        while first_day.day() != 1 {
            first_day = first_day.checked_add_days(Days::new(1)).unwrap();
            day_offset += 1;
        }

        for i in 0..self.today.num_days_in_month() as usize {
            self.render_string(
                format!("{}", i + 1).as_str(), 
                3 + horizontal_spacing * ((i + day_offset) % 7), 
                4 + vertical_spacing *   ((i + day_offset) / 7)
            );
        }

        if day_offset > 0 {
            for i in 0..day_offset {
                self.render_string(
                    format!("{}", 
                        min_week.first_day().num_days_in_month() as usize + i - day_offset + 1
                    ).as_str(), 
                    3 + horizontal_spacing * (i % 7), 
                    4 + vertical_spacing   * (i / 7)
                );
                self.screen_text[4 + vertical_spacing][i * horizontal_spacing + 1] = '┬';
            }
            
            self.screen_text[4 + vertical_spacing][1] = '┌';
            self.screen_text[4][day_offset * horizontal_spacing + 1] = '┌';
            self.color_area(Color::DarkGrey, 
                1, 
                4, 
                day_offset * horizontal_spacing, 
                vertical_spacing + 3
            )
        }

        let last_day = day_offset + min_week.last_day().num_days_in_month() as usize;
        for i in last_day..42 {
            self.render_string(
                format!("{}", i - last_day + 1).as_str(), 
                3 + horizontal_spacing * (i % 7), 
                4 + vertical_spacing   * (i / 7)
            );
        }
    }

    pub fn render_entry_tab(&mut self) {
        
    }
}
