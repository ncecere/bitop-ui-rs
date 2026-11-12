//! The bitop-ui gallery. Listens on HOST and PORT, by default 127.0.0.1:3000.

mod app;

use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match topcoat::start(app::router().build()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
