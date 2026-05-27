use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tracerust", about = "A simple ray tracer")]
pub struct Args {
    /// Scene index to render
    pub scene: u8,
}
