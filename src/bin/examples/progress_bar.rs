use bubbles::components::progress::Progress;

pub fn update_handler(p: &mut Progress) {
    p.update_value(p.value + 1);
}
pub fn create_progress_bar() -> Result<Progress<'static>, ()> {
    let root = Progress::new(
        0,
        100,
        0,
        0,
        vec!["#5A56E0", "#EE6FF8"],
        Ok(&update_handler),
    );

    Ok(root)
}
