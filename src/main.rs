mod fund;
use fund::Fund;
use std::time::SystemTime;
use std::io::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    let f = Fund{
        name: "大大",
        code: "12123",
        manager: "带带",
        v_date: "",
        v_yesterday: 0.0,
        v_today: 0.0,
        v_gap: 0.0,
        v_calc_time: ""
    };
    println!("fund: {:?}", f);
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    println!("now: {}", now);
    Ok(())
}
