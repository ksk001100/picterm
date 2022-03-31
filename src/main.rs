use eyre::Result;
use picterm::{
    app::App,
    image::{print_term_image, ImageMode},
    io::{handler::IoAsyncHandler, IoEvent},
    start_ui,
    utils::{select_mode, Mode},
};
use seahorse::{App as SeahorseApp, Context, Flag, FlagType};
use std::{env, sync::Arc};

fn main() -> Result<()> {
    let args = env::args().collect();
    let cli_app = SeahorseApp::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!(
            "{} [directory or image file]",
            env!("CARGO_PKG_NAME")
        ))
        .flag(
            Flag::new("gray", FlagType::Bool)
                .alias("g")
                .description("Gray scale mode"),
        )
        .action(action);

    cli_app.run(args);

    Ok(())
}

fn action(c: &Context) {
    match select_mode(&c.args) {
        Mode::CLI => cli_main(c),
        Mode::TUI => tui_main(c),
    }
}

fn cli_main(c: &Context) {
    let img = image::open(&c.args[0]).unwrap();
    let mode = if c.bool_flag("gray") {
        ImageMode::GrayScale
    } else {
        ImageMode::Rgba
    };

    print_term_image(img, mode);
}

fn tui_main(c: &Context) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(1000);

        let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
        let app_ui = Arc::clone(&app);

        let path = match c.args.len() {
            1 => c.args[0].clone(),
            _ => "./".to_string(),
        };

        let mode = if c.bool_flag("gray") {
            ImageMode::GrayScale
        } else {
            ImageMode::Rgba
        };

        tokio::spawn(async move {
            let mut handler = IoAsyncHandler::new(app);
            while let Some(io_event) = sync_io_rx.recv().await {
                handler.handle_io_event(io_event).await;
            }
        });

        start_ui(&app_ui, path, mode).await.unwrap();
    });
}
