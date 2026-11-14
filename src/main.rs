mod app;
mod components;
mod data;
mod pages;
mod routing;

fn main() {
    leptos::mount::mount_to_body(app::App);
}
