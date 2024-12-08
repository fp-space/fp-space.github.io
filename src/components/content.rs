// 内容展示区域

use yew::prelude::*;
use crate::debug_border;

#[function_component(MainContent)]
pub fn content() -> Html {
    html! {
                    <div class={classes!(
                        "flex-1",
                        debug_border(),
                        "m-4",
                        "p-4"
                    )}>
                        <h2 class={classes!("text-2xl", "font-bold")}>{"Main Content"}</h2>
                        <p>{"This is the main content section."}</p>
                    </div>
    }
}