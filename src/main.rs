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
        <div class="flex h-screen">
            // 左侧侧边栏
            <Sidebar />

            // 主内容区域
            <div class="flex flex-col flex-1">
                // 顶部导航栏
                <Navigation />

                // 主内容部分
                <MainContent />
            </div>
        </div>
    }
}


fn main() {
    // 初始化日志
    init_with_level(Level::Info).expect("failed to initialize logger");

    yew::Renderer::<App>::new().render();
}
