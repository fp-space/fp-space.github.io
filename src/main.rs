mod markdown;
mod utils;
mod components;

use console_log::init_with_level;
use log::Level;
use yew::{classes, function_component, html, Html};
use crate::components::header::Navigation;
use crate::components::content::MainContent;
use crate::components::sidebar::Sidebar;
use crate::components::footer::Footer;


// 控制调试模式（是否启用边框）
const DEBUG_MODE: bool = true;

// 这个函数根据 DEBUG_MODE 返回相应的边框样式
fn debug_border() -> &'static str {
    if DEBUG_MODE {
        "border-4 border-black"
    } else {
        ""
    }
}

#[function_component]
fn App() -> Html {
    html! {
        // 主容器
        <div class={classes!(
            "min-h-screen", // 设置最小高度为屏幕高度
            "flex", // 使用弹性布局
            "flex-col", // 垂直排列
            "bg-gray-100", // 背景颜色
            debug_border()  // 调试用边框
        )}>
            <Navigation /> // 导航组件

            <div class={classes!(
                "flex", // 弹性布局
                "flex-grow", // 占据剩余空间
                debug_border(),  // 调试用边框
                "p-4", // 内边距
                "m-4" // 外边距
            )}>
                // 主内容部分
                <MainContent /> // 主要内容组件

                // 侧边栏部分
                <Sidebar /> // 侧边栏组件
            </div>

            // 页脚部分
            <Footer /> // 页脚组件
        </div>
    }
}

fn main() {
    // 初始化日志
    init_with_level(Level::Info).expect("failed to initialize logger");

    yew::Renderer::<App>::new().render();
}
