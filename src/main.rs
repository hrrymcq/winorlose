use rand::Rng;

fn format_with_commas(number: f64) -> String {
    let formatted = format!("{:.2}", number);
    let mut parts = formatted.split('.');
    let integer_part = parts.next().unwrap_or("");
    let fractional_part = parts.next().unwrap_or("");

    let formatted_integer = integer_part
        .chars()
        .rev()
        .enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && i % 3 == 0 {
                Some(',')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    format!("${}.{}", formatted_integer, fractional_part)
}

fn main() {
    let mut balance = 100.0; // Starting balance
    let win_multiplier = 1.055;
    let lose_multiplier = 0.95;
    let num_rounds = 10000; // You can change this to the desired number of rounds
    let mut wins = 0;
    let mut losses = 0;
    let mut current_win_streak = 0;
    let mut current_lose_streak = 0;
    let mut max_win_streak = 0;
    let mut max_lose_streak = 0;

    println!("Starting balance: {}", format_with_commas(balance)); // Format the starting balance

    for round in 1..=num_rounds {
        let random_number = rand::thread_rng().gen_range(0..2); // 0 represents a loss, 1 represents a win

        match random_number {
            0 => {
                balance *= lose_multiplier;
                losses += 1;
                current_win_streak = 0;
                current_lose_streak += 1;
                max_lose_streak = max_lose_streak.max(current_lose_streak);
                println!(
                    "Trade #{}: Lost! New balance: {}",
                    round,
                    format_with_commas(balance) // Format the new balance
                );
            }
            1 => {
                balance *= win_multiplier;
                wins += 1;
                current_lose_streak = 0;
                current_win_streak += 1;
                max_win_streak = max_win_streak.max(current_win_streak);
                println!(
                    "Trade #{}: Won! New balance: {}",
                    round,
                    format_with_commas(balance) // Format the new balance
                );
            }
            _ => {
                println!("Invalid random number generated.");
                break;
            }
        }

        if balance <= 2.5 {
            println!("Account balance is too low.");
            break;
        }
    }

    let total_trades = wins + losses;
    let win_percentage = (wins as f64 / total_trades as f64) * 100.0;
    let loss_percentage = (losses as f64 / total_trades as f64) * 100.0;

    println!("Statistics:");
    println!("Total trades: {}", total_trades);
    println!("Wins: {} ({:.2}%)", wins, win_percentage);
    println!("Losses: {} ({:.2}%)", losses, loss_percentage);
    println!("Biggest Win Streak: {}", max_win_streak);
    println!("Biggest Lose Streak: {}", max_lose_streak);
}
