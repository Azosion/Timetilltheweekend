use chrono::{DateTime, Datelike, Local};
use fancy_print::{Animation, FancyPrinter};
use std::time::{Duration, SystemTime};
use std::env;

fn main() {

        let args: Vec<String> = env::args().skip(1).collect(); // Skip the first argument (program name)


        if args.contains(&"-i".to_string()) {

        let curr_time = SystemTime::now();

        let datetime: DateTime<Local> = curr_time.into();

        let weekdate = datetime.date_naive().weekday().to_string();

        println!(""); //I did the new lines like this to make it more readable for me.

        if weekdate == "Mon" {
                //I've separated the lines into 3 prints so that it lines up here, I could turn it all into one print tho.
            println!("   5 days till the weekend, good luck...");
            println!("---------------------------------------------");
            println!("|> Mon <    Tue      Wed      Thu      Fri  |");
        }

        if weekdate == "Tue" {
            println!("    4 days till the weekend, keep going!");
            println!("---------------------------------------------");
            println!("|  Mon    > Tue <    Wed      Thu      Fri  |");
        }

        if weekdate == "Wed" {
            
            println!("   3 days till the weekend, half way there!");
            println!("---------------------------------------------");
            println!("|  Mon      Tue    > Wed <    Thu      Fri  |");
        }

        if weekdate == "Thu" {
            
            println!("    2 days till the weekend, almost there");
            println!("---------------------------------------------");
            println!("|  Mon      Tue      Wed    > Thu <    Fri  |");
        }

        if weekdate == "Fri" {
            
            println!("1 day till the weekend, last day, u got this!");
            println!("---------------------------------------------");
            println!("|  Mon      Tue      Wed      Thu    > Fri <|");
        }

        if weekdate == "Sat" || weekdate == "Sun" {
            
            //TODO: add more flair to this msg on Sat and Sun 

            println!("It's the weekend, you made it!");
            
        }
        println!("");

    } 

    else {

        let curr_time = SystemTime::now();

        let datetime: DateTime<Local> = curr_time.into();

        let weekdate = datetime.date_naive().weekday().to_string();

        let printer = FancyPrinter::builder()
            .animation(Animation::Typing)
            .time_delay(Duration::from_millis(20))
            .build();

        println!(""); //I did the new lines like this to make it more readable for me.

        if weekdate == "Mon" {
            //I've separated the lines into 3 prints so that it lines up here, I could turn it all into one print tho.
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
            
            printer.print("    2 days till the weekend, almost there");
            printer.print("---------------------------------------------");
            printer.print("|  Mon      Tue      Wed    > Thu <    Fri  |");
        }

        if weekdate == "Fri" {
            
            printer.print("1 day till the weekend, last day, u got this!");
            printer.print("---------------------------------------------");
            printer.print("|  Mon      Tue      Wed      Thu    > Fri <|");
        }

        if weekdate == "Sat" || weekdate == "Sun" {
            
            //TODO: add more flair to this msg on Sat and Sun 

            printer.print("It's the weekend, you made it!");
            
        }
        println!("");

    }
}
