use colored::*;

fn main() {
    println!(
        "\n{}",
        "OSTBMMO server started. Beginning tick loop..."
            .green()
            .italic()
    );

    let mut tick: u64 = 0;

    // Game Event Loop
    loop {
        tick += 1;
        println!("{}: {}", "Tick:".blue().bold(), tick.to_string().yellow());
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

