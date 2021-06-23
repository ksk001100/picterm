use eyre::Result;
use picterm::app::App;
use picterm::io::handler::IoAsyncHandler;
use picterm::io::IoEvent;
use picterm::start_ui;
use std::path::Path;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    check_args();

    let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(100);

    let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
    let app_ui = Arc::clone(&app);

    tokio::spawn(async move {
        let mut handler = IoAsyncHandler::new(app);
        while let Some(io_event) = sync_io_rx.recv().await {
            handler.handle_io_event(io_event).await;
        }
    });

    start_ui(&app_ui).await?;

    Ok(())
}

fn check_args() {
    let args: Vec<String> = std::env::args().collect();
    match args[1..].len() {
        0 => (),
        1 => {
            if !Path::new(&args[1]).is_dir() {
                eprintln!("Please specify only one directory as an argument");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Please specify only one directory as an argument");
            std::process::exit(1);
        }
    }
}
