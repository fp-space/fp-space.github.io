// 导航栏组件
use yew::prelude::*;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <div class="bg-blue-600 text-white px-4 py-3 shadow-md">
            <h1 class="text-lg font-semibold">{"Welcome to Mini Note!"}</h1>
        </div>
    }
}
