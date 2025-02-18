use chrono::{DateTime, Datelike, Utc};
use fancy_print::{Animation, FancyPrinter};
use std::time::{Duration, SystemTime};

fn main() {
    let curr_time = SystemTime::now();

    let datetime: DateTime<Utc> = curr_time.into();

    let weekdate = datetime.date_naive().weekday().to_string();

    let printer = FancyPrinter::builder()
        .animation(Animation::Typing)
        .time_delay(Duration::from_millis(20))
        .build();

    println!(""); //I did it this way to make it easier to read for me. 

    if weekdate == "Mon" {
         
        printer.print("   5 days till the weekend, good luck...");
        printer.print("---------------------------------------------");
        printer.print("|> Mon <    Tue      Wed      Thu      Fri  |");
    }

    if weekdate == "Tue" {
        printer.print("    4 days till the weekend, keep going!");
        printer.print("---------------------------------------------");
        printer.print("|  Mon    > Tue <    Wed      Thu      Fri  |");
    }

    if weekdate == "Wed" {
        
        printer.print("   3 days till the weekend, half way there!");
        printer.print("---------------------------------------------");
        printer.print("|  Mon      Tue    > Wed <    Thu      Fri  |");
    }

    if weekdate == "Thu" {
        
        printer.print("   2 days till the weekend, almost there");
        printer.print("---------------------------------------------");
        printer.print("|  Mon      Tue      Wed    > Thu <    Fri  |");
    }

    if weekdate == "Fri" {
        
        printer.print("1 day till the weekend, last day... u got this!");
        printer.print("---------------------------------------------");
        printer.print("|  Mon      Tue      Wed      Thu    > Fri <|");
    }

    if weekdate == "Sat" || weekdate == "Sun" {
        
        printer.print("It's the weekend, you made it!");
        
    }
    println!("");
}
