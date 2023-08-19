mod food_choice;
mod queries;

fn main() -> color_eyre::Result<()>{
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    Ok(())
}