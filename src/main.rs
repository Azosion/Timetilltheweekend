
use fancy_print::{Animation, FancyPrinter};
use chrono::{Utc, DateTime, Datelike};
use std::time::{Duration, SystemTime};


fn main() {
   let curr_time = SystemTime::now();

   let datetime: DateTime<Utc> = curr_time.into();

   let weekdate = datetime.date_naive().weekday().to_string();

   let printer = FancyPrinter::builder()
     .animation(Animation::Typing)
     .time_delay(Duration::from_millis(20))
     .build();

   if weekdate == "Mon" {
          println!("");
          printer.print("   5 days till the weekend, good luck...");        
          printer.print("---------------------------------------------");
          printer.print("|> Mon <    Tue      Wed      Thu      Fri  |");
          println!("");
   }

   if weekdate == "Tue" {
          println!("");
          printer.print("    4 days till the weekend, keep going!");
          printer.print("---------------------------------------------");
          printer.print("|  Mon    > Tue <    Wed      Thu      Fri  |");
          println!("");
          
   }

   if weekdate == "Wed" {
          println!("");
          printer.print("   3 days till the weekend, half way there!");
          printer.print("---------------------------------------------");
          printer.print("|  Mon      Tue    > Wed <    Thu      Fri  |");
          println!("");
   }

   if weekdate == "Thu" {
          println!("");
          printer.print("   2 days till the weekend, almost there");
          printer.print("---------------------------------------------");
          printer.print("|  Mon      Tue      Wed    > Thu <    Fri  |");
          println!("");
   }

   if weekdate == "Fri" {
          println!("");
          printer.print("1 day till the weekend, last day... u got this!");
          printer.print("---------------------------------------------");
          printer.print("|  Mon      Tue      Wed      Thu    > Fri <|");
          println!("");
   }

}