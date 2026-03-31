mod telemetry;
mod plot;

use clap::Parser;

/// Plot FH engine output at wheel to vector graphics (SVG)
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Data out IP address 
    host: String,

    /// Data out port
    port: u16,

    /// Width of the entire plot
    #[arg(short = 'W', long, default_value = "800")]
    width: u32,

    /// Height of the entire plot
    #[arg(short = 'H', long, default_value = "600")]
    height: u32,

    /// location for storing the image
    #[arg(default_value = "output.svg")]
    path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("listening for the game on {}:{}", args.host, args.port);
    let data = crate::telemetry::recv((args.host, args.port)).await?;
    crate::plot::write_svg_to(&args.path, (args.width, args.height), data)?;

    Ok(())
}
