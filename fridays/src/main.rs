use chrono::prelude::*;

fn main() {
    let mut day_counts = [0; 7];

    for year in 1600..2000 {
        for month in 1..13 {
            let day_one = (11 - Utc.ymd(year, month, 1).weekday().num_days_from_monday()) % 7;
            day_counts[day_one as usize] += 1;
        }
    }
    println!("Counts of fridays by daynumber");
    println!("==============================");
    for (day, count) in day_counts.iter().enumerate() {
        println!("{}: {}", day + 8, count);
    }
}
