// 目录树组件

use yew::prelude::*;
use crate::debug_border;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
                    <div class={classes!(
                        "w-64",
                        debug_border(),
                        "m-4",
                        "p-4"
                    )}>
                        <h3 class={classes!("text-xl", "font-semibold")}>{"Sidebar"}</h3>
                        <p>{"This is the sidebar content."}</p>
                    </div>
    }
}