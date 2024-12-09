// 导航栏组件
use yew::prelude::*;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <div class="nav-container">
            <h1 class="nav-title">{"Welcome to Mini Note!"}</h1>
        </div>
    }
}
