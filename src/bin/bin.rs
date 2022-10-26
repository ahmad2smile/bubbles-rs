use bubbles::{self, app::App};
use examples::{input_field::create_input_field, progress_bar::create_progress_bar};

pub mod examples;

fn main() {
    let mut app = App::new();

    let progress = create_progress_bar().unwrap();
    let input = create_input_field().unwrap();

    // NOTE: Method overloading based on different Traits didn't work as `root` impl multiple traits
    // app.register_render(&mut root);
    // app.register_events(x);
    app.register(Box::new(progress));
    app.register(Box::new(input));

    let result = app.run();

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }
}
