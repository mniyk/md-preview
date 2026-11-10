use std::time::Duration;

use leptos::prelude::*;
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use leptos::wasm_bindgen::prelude::*;

#[wasm_bindgen(wasm_bindgen = leptos::wasm_bindgen)]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = renderMermaid)]
    fn render_mermaid();
}

fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

fn md_to_html(src: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    // ```mermaid ブロックを <pre class="mermaid"> に差し替える
    let mut events = Vec::new();
    let mut mermaid: Option<String> = None;
    for ev in Parser::new_ext(src, options) {
        match ev {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang)))
                if lang.as_ref() == "mermaid" =>
            {
                mermaid = Some(String::new());
            }
            Event::Text(ref t) if mermaid.is_some() => {
                mermaid.as_mut().unwrap().push_str(t);
            }
            Event::End(TagEnd::CodeBlock) if mermaid.is_some() => {
                let code = mermaid.take().unwrap();
                events.push(Event::Html(CowStr::from(format!(
                    "<pre class=\"mermaid\">{}</pre>\n",
                    escape_html(&code)
                ))));
            }
            ev => events.push(ev),
        }
    }

    let mut out = String::new();
    html::push_html(&mut out, events.into_iter());
    out
}

#[component]
fn App() -> impl IntoView {
    let (text, set_text) = signal(String::new());
    let html = Memo::new(move |_| md_to_html(&text.get()));

    // プレビュー更新後、入力が落ち着いてから Mermaid を描画する
    let pending = StoredValue::new(None::<TimeoutHandle>);
    Effect::new(move |_| {
        html.track();
        if let Some(h) = pending.get_value() {
            h.clear();
        }
        let handle = set_timeout_with_handle(render_mermaid, Duration::from_millis(100)).ok();
        pending.set_value(handle);
    });

    view! {
        <div class="container">
            <textarea
                class="editor"
                on:input=move |ev| set_text.set(event_target_value(&ev))
            />
            <div class="preview" inner_html=move || html.get() />
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
