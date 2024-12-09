// 内容展示区域

use yew::prelude::*;

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <div>
            <h2>{"Main Content Section"}</h2>
            <p>{"This is the main content area."}</p>
        </div>
    }
}
