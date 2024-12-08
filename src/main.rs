mod markdown;
mod utils;
mod components;

use console_log::init_with_level;
use log::Level;
use yew::{classes, function_component, html, Html};
use crate::components::header::Navigation;


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
            <div class={classes!(
                "min-h-screen",
                "flex",
                "flex-col",
                "bg-gray-100",
                debug_border()  // 使用 debug_border 函数
            )}>
                <Navigation />

                <div class={classes!(
                    "flex",
                    "flex-grow",
                    debug_border(),  // 使用 debug_border 函数
                    "p-4",
                    "m-4"
                )}>
                    <div class={classes!(
                        "flex-1",
                        debug_border(),  // 使用 debug_border 函数
                        "m-4",
                        "p-4"
                    )}>
                        <h2 class={classes!("text-2xl", "font-bold")}>{"Main Content"}</h2>
                        <p>{"This is the main content section."}</p>
                    </div>

                    <div class={classes!(
                        "w-64",
                        debug_border(),  // 使用 debug_border 函数
                        "m-4",
                        "p-4"
                    )}>
                        <h3 class={classes!("text-xl", "font-semibold")}>{"Sidebar"}</h3>
                        <p>{"This is the sidebar content."}</p>
                    </div>
                </div>

                <div class={classes!(
                    debug_border(),  // 使用 debug_border 函数
                    "p-4",
                    "text-center"
                )}>
                    <p>{"This is the footer section."}</p>
                </div>
            </div>
    }
}

fn main() {
    // 初始化日志
    init_with_level(Level::Info).expect("failed to initialize logger");

    yew::Renderer::<App>::new().render();
}
