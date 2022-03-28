use eyre::Result;
use picterm::{
    app::App,
    image::print_term_image,
    io::{handler::IoAsyncHandler, IoEvent},
    start_ui,
    utils::{select_mode, Mode},
};
use seahorse::{App as SeahorseApp, Context};
use std::{env, sync::Arc};

fn main() -> Result<()> {
    let args = env::args().collect();
    let cli_app = SeahorseApp::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [directory or image file]", env!("CARGO_PKG_NAME")))
        .action(action);

    cli_app.run(args);

    Ok(())
}

fn action(c: &Context) {
    match select_mode(&c.args) {
        Mode::CLI => cli_main(&c.args[0]),
        Mode::TUI => tui_main(),
    }
}

fn cli_main(path: &String) {
    let img = image::open(path).unwrap();
    print_term_image(img);
}

fn tui_main() {
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
