#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Io: {0}")]
    // #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("bar error")]
    Bar,
}

fn main() -> anyhow::Result<()> {
    run()?;
    Ok(())
}

fn run() -> anyhow::Result<()> {
    use anyhow::Context;

    if std::env::args().any(|arg| arg == "--bar") {
        try_bar().context("run step failed")?;
    } else {
        try_read().context("run step failed")?;
    }

    Ok(())
}

fn try_bar() -> Result<(), AppError> {
    Err(AppError::Bar)
}

fn try_read() -> Result<String, AppError> {
    let content = std::fs::read_to_string("notfound.toml")?;
    Ok(content)
}
