use std::process;
use anyhow::Result;

pub enum ExitCode {
    Ok,
    BasicError,
    UnexpectedError,
}

impl ExitCode {
    // The exit code 2 is used for the arguments parsing error thrown by clap.
    pub fn value(&self) -> i32 {
        match self {
            Self::Ok => 0,
            Self::BasicError => 1,
            Self::UnexpectedError => 99,
        }
    }
}

pub trait HandleAndExit {
    fn handle_and_exit(self, tag: &str);
    fn handle_and_exit_with(self, tag: &str, code_on_err: ExitCode);
}

impl HandleAndExit for Result<ExitCode> {
    fn handle_and_exit(self, tag: &str) {
        self.handle_and_exit_with(tag, ExitCode::UnexpectedError);
    }

    fn handle_and_exit_with(self, tag: &str, code_on_err: ExitCode) {
        let code = self.unwrap_or_else(|e| {
            eprintln!("{tag}: {e}");
            code_on_err
        });
        process::exit(code.value());
    }
}