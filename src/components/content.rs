// 内容展示区域

use wasm_bindgen::JsValue;
use yew::prelude::*;
use crate::context::app_context::AppStateContext;

#[function_component(MainContent)]
pub fn main_content() -> Html {

    // 监听 AppStateContext 的变化
    let app_state_ctx = use_context::<AppStateContext>();

    if let Some(app_state_ctx) = app_state_ctx {
        if let Some(ref file) = app_state_ctx.selected_file {
            web_sys::console::log_1(&JsValue::from_str(&format!("Selected File: {}", file.to_string())));
        } else {
            web_sys::console::log_1(&JsValue::from_str("No file selected"));
        }
    } else {
        // 没有找到 AppStateContext 上下文时的处理逻辑
        web_sys::console::log_1(&JsValue::from_str("AppStateContext not found"));
    }

    
    html! {
        <div class="px-40 py-10">
            <article class="prose max-w-none">

                <h1> {"Garlic bread with cheese: What the science tells us"} </h1>
                <p>
                    {"For years parents have espoused the health benefits of eating garlic bread with cheese to their
                    children, with the food earning such an iconic status in our culture that kids will often dress
                    up as warm, cheesy loaf for Halloween."}
                </p>
                <p>
                    {"But a recent study shows that the celebrated appetizer may be linked to a series of rabies cases
                    springing up around the country."}
                </p>
            </article>
        </div>

    }
}
