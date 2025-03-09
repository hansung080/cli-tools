use std::process;
use anyhow::Result;

pub enum ExitCode {
    Ok,
    ErrKeepRunning,
    ErrStopRunning,
}

impl ExitCode {
    pub fn value(&self) -> i32 {
        match self {
            Self::Ok => 0,
            Self::ErrKeepRunning => 1,
            Self::ErrStopRunning => 2,
        }
    }
}

pub trait HandleAndExit {
    fn handle_and_exit(self, tag: &str);
}

impl HandleAndExit for Result<ExitCode> {
    fn handle_and_exit(self, tag: &str) {
        let code = self.unwrap_or_else(|e| {
            eprintln!("{tag}: {e}");
            ExitCode::ErrStopRunning
        });
        process::exit(code.value());
    }
}