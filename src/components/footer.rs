// 页脚组件

use yew::prelude::*;
use crate::debug_border;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
                <div class={classes!(
                    debug_border(),
                    "p-4",
                    "text-center"
                )}>
                    <p>{"This is the footer section."}</p>
                </div>
    }
}