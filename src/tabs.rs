use crossterm::style::Color;
use chrono::{Datelike, Weekday};
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
        let vertical_spacing: usize = (self.height - 6) / 7;

        self.screen_text[4][1] = '┌';
        self.screen_text[4][self.width - 3] = '┐';
        
        for i in 2..(self.width - 3) {
            self.screen_text[4][i] = '─';
        }

        for i in 5..(vertical_spacing * 7 + 4) {
            self.screen_text[i][1] = '│';
            self.screen_text[i][self.width - 3] = '│';
        }
        
        for i in 1..7 {
            let row = vertical_spacing * (i) + 4;
            for j in 10..(self.width - 3) {
                self.screen_text[row][j] = '─';
            }
            self.screen_text[row][1] = '├';
            self.screen_text[row][self.width - 3] = '┤';
        }

        for i in 2..(self.width - 3) {
            self.screen_text[vertical_spacing * 7 + 4][i] = '─';
        }
        self.screen_text[vertical_spacing * 7 + 4][1] = '└';
        self.screen_text[vertical_spacing * 7 + 4][self.width - 3] = '┘';

        self.render_string(" Sunday ",    2, 4);
        self.render_string(" Monday ",    2, vertical_spacing * 1 + 4);
        self.render_string(" Tuesday ",    2, vertical_spacing * 2 + 4);
        self.render_string(" Wednesday ", 2, vertical_spacing * 3 + 4);
        self.render_string(" Thursday ",  2, vertical_spacing * 4 + 4);
        self.render_string(" Friday ",    2, vertical_spacing * 5 + 4);
        self.render_string(" Saturday ",  2, vertical_spacing * 6 + 4);

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
        self.color_area(Color::Rgb{r: 255, g: 200, b: 50 }, 2 + width, y, self.width - 4, y);

        let mut tasks_by_weekday: [Vec<Task>; 7] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for i in 0..self.tasks.len() {
            if self.tasks[i].date.week(Weekday::Sun) == self.today.week(Weekday::Sun) {
                let task_weekday = match self.tasks[i].date.weekday(){
                    Weekday::Sun => 0,
                    Weekday::Mon => 1,
                    Weekday::Tue => 2,
                    Weekday::Wed => 3,
                    Weekday::Thu => 4,
                    Weekday::Fri => 5,
                    Weekday::Sat => 6,
                };
                tasks_by_weekday[task_weekday].push(self.tasks[i].clone());
            }
        }

        for weekday in 0..7 {
            let y = weekday * vertical_spacing + 4 + (vertical_spacing / 2);
            let tasks = &tasks_by_weekday[weekday];
            if tasks.len() == 1 {
                self.render_string("██ ", 3, y);
                self.render_string(&tasks[0].description, 6, y);
                self.color_area(task_color(&tasks[0]), 3, y, 4, y);  
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
                let mut x = 3;
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
        
    }

    pub fn render_entry_tab(&mut self) {
        
    }
}