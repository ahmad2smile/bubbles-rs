use bubbles;

fn main() {
    let root = bubbles::components::progress::Progress::new(100, vec!["#5A56E0", "#EE6FF8"]);

    bubbles::app::run(&root).unwrap();
}
