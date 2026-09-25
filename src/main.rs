use leptos::prelude::*;
use pulldown_cmark::{html, Options, Parser};

fn md_to_html(src: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let mut out = String::new();
    html::push_html(&mut out, Parser::new_ext(src, options));
    out
}

#[component]
fn App() -> impl IntoView {
    let (text, set_text) = signal(String::new());

    view! {
        <div class="container">
            <textarea
                class="editor"
                on:input=move |ev| set_text.set(event_target_value(&ev))
            />
            <div class="preview" inner_html=move || md_to_html(&text.get()) />
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
