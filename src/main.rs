
mod utils;
mod components;
mod model;
mod context;


use crate::components::content::MainContent;
use crate::components::navigation::Navigation;
use crate::components::sidebar::Sidebar;
use console_log::init_with_level;
use log::Level;
use yew::{function_component, html, use_effect, Html, Renderer};
use crate::context::app_context::AppStateProvider;
use crate::components::script::render_mathjax;


#[function_component(App)]
fn app() -> Html {

    // 在组件挂载时动态插入 MathJax 脚本及配置
    use_effect(|| {
        render_mathjax();
    });


    html! {
        <AppStateProvider>
            <div class="app-container">
                <div class="app-container-sidebar">
                    <Sidebar />
                </div>

                <div class="main-content">
                    <div class="main-content-navbar">
                        <Navigation />
                    </div>

                    <div class="main-content-area">
                        <MainContent />
                    </div>
                </div>
            </div>
        </AppStateProvider>
    }
}


fn main() {
    // Initialize logging for WASM
    init_with_level(Level::Info).expect("failed to initialize logger");

    // Render the Yew app
    Renderer::<App>::new().render();
}