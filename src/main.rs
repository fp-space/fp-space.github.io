mod markdown;
mod utils;
mod components;

use crate::components::content::MainContent;
use crate::components::navigation::Navigation;
use crate::components::sidebar::Sidebar;
use console_log::init_with_level;
use log::Level;
use yew::{function_component, html, Html};

#[function_component(App)]
fn app() -> Html {
    html! {
        // 主容器
        <div class="app-container">
            <Navigation /> // 导航组件

            <div class="main-content-container">

                // 侧边栏部分
                <Sidebar /> // 侧边栏组件

                // 主内容部分
                <MainContent /> // 主要内容组件
            </div>
        </div>
    }
}
fn main() {
    // 初始化日志
    init_with_level(Level::Info).expect("failed to initialize logger");

    yew::Renderer::<App>::new().render();
}
