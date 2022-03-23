use eyre::Result;
use picterm::app::App;
use picterm::io::handler::IoAsyncHandler;
use picterm::io::IoEvent;
use picterm::start_ui;
use seahorse::{App as SeahorseApp, Context};
use std::env;
use std::path::Path;
use std::sync::Arc;

fn main() -> Result<()> {
    let args = env::args().collect();
    let cli_app = SeahorseApp::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [directory]", env!("CARGO_PKG_NAME")))
        .action(tui_main);

    cli_app.run(args);

    Ok(())
}

fn tui_main(c: &Context) {
    check_args(&c.args);

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(1000);

        let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
        let app_ui = Arc::clone(&app);

        tokio::spawn(async move {
            let mut handler = IoAsyncHandler::new(app);
            while let Some(io_event) = sync_io_rx.recv().await {
                handler.handle_io_event(io_event).await;
            }
        });

        start_ui(&app_ui).await.unwrap();
    });
}

fn check_args(args: &Vec<String>) {
    match args.len() {
        0 => (),
        1 => {
            if !Path::new(&args[0]).is_dir() {
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
