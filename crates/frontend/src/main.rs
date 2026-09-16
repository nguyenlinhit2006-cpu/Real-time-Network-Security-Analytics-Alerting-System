mod api;
mod app;
mod components;
mod pages;
mod ws;

use app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
