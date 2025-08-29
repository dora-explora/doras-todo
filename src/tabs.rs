use crossterm::style::Color;
use chrono::{Datelike, Weekday};
use crate::{App, Task, Subject, X_MAX, Y_MAX};

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

        self.render_string(" Sunday ",    4, HORIZONTAL_SPACING * 1 + 1);
        self.render_string(" Monday ",    4, HORIZONTAL_SPACING * 2 + 1);
        self.render_string(" Tueday ",    4, HORIZONTAL_SPACING * 3 + 1);
        self.render_string(" Wednesday ", 4, HORIZONTAL_SPACING * 4 + 1);
        self.render_string(" Thursday ",  4, HORIZONTAL_SPACING * 5 + 1);
        self.render_string(" Friday ",    4, HORIZONTAL_SPACING * 6 + 1);
        self.render_string(" Saturday ",  4, HORIZONTAL_SPACING * 7 + 1);

        let (y, width) = match self.today.weekday(){
            Weekday::Sun => (5, 8),
            Weekday::Mon => (9, 8),
            Weekday::Tue => (13, 8),
            Weekday::Wed => (17, 11),
            Weekday::Thu => (21, 10),
            Weekday::Fri => (25, 8),
            Weekday::Sat => (29, 10),
        };
        self.color_area(Color::Rgb{r: 255, g: 200, b: 50 }, 4 + width, y, X_MAX - 6, y);

        let mut tasks_by_weekday: [Vec<Task>; 7] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for i in 0..self.tasks.len() {
            let task = self.tasks[i].clone();
            if task.date.week(Weekday::Sun) == self.today.week(Weekday::Sun) {
                let weekday: usize = match task.date.weekday() {
                    Weekday::Sun => 0,
                    Weekday::Mon => 1,
                    Weekday::Tue => 2,
                    Weekday::Wed => 3,
                    Weekday::Thu => 4,
                    Weekday::Fri => 5,
                    Weekday::Sat => 6,
                };
                tasks_by_weekday[weekday].push(task);
            }
        }

        for weekday in 0..6 {
            let y = weekday * 4 + 7;
            let tasks = &tasks_by_weekday[weekday];
            if tasks.len() == 1 {
                self.render_string("██ ", 5, y);
                self.render_string(&tasks[0].description, 8, y);
                self.color_area(task_color(&tasks[0]), 5, y, 6, y);  
            } else if tasks.len() > 1 {
                for i in 0..tasks.len() {
                    if i == 0 {
                        self.render_string("██ ", 5, y);
                        self.render_string(&tasks[0].description, 8, y);
                        self.color_area(task_color(&tasks[0]), 5, y, 6, y);
                    } else {
                        let mut x = 5;
                        for j in 0..i {
                            x += 6;
                            x += tasks[j].description.len();
                        }
                        self.screen_text[y][x - 2] = '│';
                        self.render_string("██ ", x, y);
                        self.render_string(&tasks[i].description, x + 3, y);
                        self.color_area(task_color(&tasks[i]), x, y, x + 1, y);
                    }
                }
            }
        }

        // let y = match task.date.weekday() {
        //     Weekday::Sun => 7,
        //     Weekday::Mon => 11,
        //     Weekday::Tue => 15,
        //     Weekday::Wed => 19,
        //     Weekday::Thu => 23,
        //     Weekday::Fri => 27,
        //     Weekday::Sat => 31,
        // };
        // self.render_string("██ ", 5, y);
        // self.render_string(&task.description, 8, y);
        // self.color_area(task_color(&task), 5, y, 6, y);
    }

    pub fn render_month_tab(&mut self) {
        
    }

    pub fn render_entry_tab(&mut self) {
        
    }
}