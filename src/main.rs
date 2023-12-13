//! Client-side web application for zstewart.com

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    const MAIN: &'static str = include_str!("main.rs");
    const JS: &'static str = include_str!("reprocess.js");
    use_effect(|| run_hljs());
    html! {
        <div>
            <pre class="rust"><code>
                {MAIN}
            </code></pre>
            <pre class="javascript"><code>
                {JS}
            </code></pre>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[wasm_bindgen(module = "/src/reprocess.js")]
extern "C" {
    /// JS function def that runs hljs.highlightAll.
    pub fn run_hljs();
}
