mod app;

use app::TauriApp;
use app::TauriAppProps;
use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx,  <TauriApp /> });
}
