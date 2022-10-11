use bubbles;

fn main() {
    let root = bubbles::components::progress::Progress::default();

    bubbles::app::run(&root).unwrap();
}
