use crate::fund::Fund;
use termion::color;

pub struct Blueprint {
    funds: Vec<Fund>,
}

impl Blueprint {
    pub fn new(funds: Vec<Fund>) -> Self {
        Blueprint { funds }
    }

    pub fn draw(&self) {
        for f in self.funds.iter() {
            if f.v_gap.eq("") {
                continue
            }

            if f.v_gap.starts_with('-') {
                println!("{}{}", color::Fg(color::Green), f);
            } else {
                println!("{}{}", color::Fg(color::Red), f);
            };
        }
    }
}
