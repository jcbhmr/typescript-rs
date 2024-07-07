#![doc(hidden)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const TYPESCRIPT_VERSION: &str = "5.5.3";
    let sh = xshell::Shell::new()?;
    eprintln!("TYPESCRIPT_VERSION={:?}", TYPESCRIPT_VERSION);
    let pack_destination = std::env::temp_dir();
    let output = xshell::cmd!(
        sh,
        "npm pack typescript@{TYPESCRIPT_VERSION} --pack-destination {pack_destination}"
    )
    .output()?;
    let out_relative = std::str::from_utf8(&output.stdout)?.trim();
    let out = pack_destination.join(out_relative);
    let dest = "typescript.tgz";
    std::fs::rename(out, dest)?;
    eprintln!("generated {dest:?}");
    Ok(())
}
