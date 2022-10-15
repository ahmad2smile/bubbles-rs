use bubbles::{self, app::App};
use examples::progress_bar::create_progress_bar;

pub mod examples;

fn main() {
    let app = App::new();

    let mut root = create_progress_bar().unwrap();

    let result = app.run(&mut root);

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }
}
