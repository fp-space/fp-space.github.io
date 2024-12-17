mod markdown;
mod utils;
mod components;
mod model;
mod context;

use crate::components::content::MainContent;
use crate::components::navigation::Navigation;
use crate::components::sidebar::Sidebar;
use console_log::init_with_level;
use log::Level;
use yew::{function_component, html, Html, Renderer};
use crate::context::app_context::AppStateProvider;

// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         <AppStateProvider>
//             <div class="flex h-screen">
//                 // 左侧侧边栏
//                 <div class="w-64 bg-gray-50 shadow-lg overflow-y-auto">
//                     <Sidebar />
//                 </div>
//
//                 // 右侧内容区域
//                 <div class="flex flex-col flex-1">
//                     // 顶部导航栏
//                     <div class="h-16 bg-white shadow-md flex items-center px-4">
//                         <Navigation />
//                     </div>
//
//                     // 主内容部分
//                     <div class="flex-1 overflow-y-auto p-6">
//                         <MainContent />
//                     </div>
//                 </div>
//             </div>
//         </AppStateProvider>
//     }
// }


#[function_component(App)]
fn app() -> Html {
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