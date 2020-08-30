#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    phrases: Vec<String>,
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg = confy::load("warmfuzzies")?;
    println!("{:?}", cfg.phrases);
    Ok(())
}
