fn main() {
    let time = "12:05:45AM"; //change to 00:05:45
    let am_or_pm = &time[8..];
    let hour = &time[..2];
    let min_and_secs = &time[2..8];
    if am_or_pm == "PM" {
        let hours = &hour.trim().parse().unwrap() + 12 % 12;
        // hours  =     hours + 12;
        println!("hour is {} and hours is {}", hour, hours);
        println!("The time is {}{}", hours, min_and_secs);
    } else {
        let hours = (&hour.trim().parse().unwrap()) % 12;
        println!("The is {}{}", hours, min_and_secs);
    }
}
