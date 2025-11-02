use clap::Parser as _;

fn main() -> anyhow::Result<()> {
    let args = cargo_verbump::Args::parse();

    cargo_verbump::update_version(args)
}
