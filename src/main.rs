use chess_engine_rs::application::Application;

fn main() {
    let mut app = Application::default();
    app.listen();
    loop {
        app.process();
    }
}