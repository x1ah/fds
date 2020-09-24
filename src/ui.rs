use crate::fund::Fund;
use prettytable::{format, Cell, Row, Table};

pub struct Blueprint {
    funds: Vec<Fund>,
}

impl Blueprint {
    pub fn new(funds: Vec<Fund>) -> Self {
        Blueprint { funds }
    }

    pub fn draw(&self) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        for f in self.funds.iter() {
            if f.v_gap.eq("") {
                continue;
            }
            let style = if f.v_gap.starts_with('-') {
                "Fgc"
            } else {
                "Frc"
            };

            table.add_row(Row::new(vec![
                Cell::new(&f.code).style_spec(style),
                Cell::new(&f.name).style_spec(style),
                Cell::new(&f.v_gap).style_spec(style),
                Cell::new(&f.v_calc_time).style_spec(style),
                Cell::new(&f.manager).style_spec(style),
            ]));
        }
        if !table.is_empty() {
            table.insert_row(
                0,
                Row::new(vec![
                    Cell::new("基金代码").style_spec("c"),
                    Cell::new("基金名称").style_spec("c"),
                    Cell::new("估值").style_spec("c"),
                    Cell::new("估值时间").style_spec("c"),
                    Cell::new("基金经理").style_spec("c"),
                ]),
            );
            table.printstd();
        }
    }
}
