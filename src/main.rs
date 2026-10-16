mod ep10;
mod ep20;
mod ep30;
mod ep40;
mod ep50;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_default();

    match arg.as_str() {
        "10" | "ep10" => ep10::ep10(),
        "20" | "ep20" => ep20::ep20(),
        "30" | "ep30" => ep30::ep30(),
        "40" | "ep40" => ep40::ep40(),
        "50" | "ep50" => ep50::ep50(),
        _ => {
            eprintln!("Usage: learn-bevy <episode>");
            eprintln!("Available episodes: 10, 20, 30");
            std::process::exit(1);
        }
    }
}
