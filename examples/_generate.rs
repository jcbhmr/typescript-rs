#![doc(hidden)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sh = xshell::Shell::new()?;
    sh.change_dir("crates/typescript-sys");
    xshell::cmd!(sh, "cargo run --example _generate").run()?;
    Ok(())
}
