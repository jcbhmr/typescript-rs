#[cfg(not(windows))]
const EXE_EXT: &str = "";
#[cfg(windows)]
const EXE_EXT: &str = ".exe";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const TSC: &[u8] = typescript_linux_x86_64::TSC;
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const TSC: &[u8] = typescript_windows_x86_64::TSC_EXE;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write("tsc" + EXE_EXT, TSC)?;
    std::process::Command::new(tsc + EXE_EXT)
        .args(std::env::args().skip(1))
        .status()?;
    Ok(())
}
