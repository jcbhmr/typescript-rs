#[cfg(test)]
mod tests {
    const TYPESCRIPT_VERSION: &str = "5.5.2";
    const TARGET: &str = "x86_64-unknown-linux-gnu";

    #[test]
    fn tsc() -> Result<(), Box<dyn std::error::Error>> {
        let sh = xshell::Shell::new()?;
        eprintln!("TYPESCRIPT_VERSION={:?}", TYPESCRIPT_VERSION);
        eprintln!("TARGET={:?}", TARGET);
        _ = xshell::cmd!(
            sh,
            "deno compile -A --target {TARGET} npm:typescript@{TYPESCRIPT_VERSION}/tsc"
        )
        .run()?;
        Err("successful codegen. rerun without codegen.".into())
    }

    #[test]
    fn tsserver() -> Result<(), Box<dyn std::error::Error>> {
        let sh = xshell::Shell::new()?;
        eprintln!("TYPESCRIPT_VERSION={:?}", TYPESCRIPT_VERSION);
        eprintln!("TARGET={:?}", TARGET);
        _ = xshell::cmd!(
            sh,
            "deno compile -A --target {TARGET} npm:typescript@{TYPESCRIPT_VERSION}/tsserver"
        )
        .run()?;
        Err("successful codegen. rerun without codegen.".into())
    }
}

const TSC: &[u8] = include_bytes!("../tsc");
const TSSERVER: &[u8] = include_bytes!("../tsserver");
