use chrono::{Local, offset::FixedOffset}; // Only import necessary modules

fn main() {
    let name = "Muhammed Althaf"; // Your name

    // Create an Indian Standard Time (IST) timezone offset (+5 hours 30 minutes)
    let ist_offset = FixedOffset::east_opt(5 * 3600 + 1800).unwrap(); // +5:30
    
    // Get the current time in IST
    let current_time = Local::now().with_timezone(&ist_offset);

    // Print the custom greeting with the time
    println!("Hello {}, right now the time is {} in IST.", 
             name, 
             current_time.format("%Y-%m-%d %H:%M:%S"));

    // Adding something extra
    println!("Hope you're having a productive day! Keep coding and stay awesome 😎!");
}

