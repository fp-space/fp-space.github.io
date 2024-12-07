mod markdown;
mod utils;

use console_log::init_with_level;
use log::Level;
use yew::{function_component, html, Html};


#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{"Hello Mini Note"}</h1>
        </div>
    }
}

fn main() {
    // 初始化日志
    init_with_level(Level::Info).expect("failed to initialize logger");

    yew::Renderer::<App>::new().render();
}
