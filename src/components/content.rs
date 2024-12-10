// 内容展示区域

use yew::prelude::*;

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <div class="flex-1 bg-gray-100 p-6">
            <h2 class="text-2xl font-bold mb-4">{"Main Content Section"}</h2>
            <p>{"This is the main content area, located below the navigation bar."}</p>
        </div>
    }
}
