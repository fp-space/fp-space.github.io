// // 导航栏组件
// use yew::prelude::*;
//
// #[function_component(Navigation)]
// pub fn navigation() -> Html {
//     html! {
//         <div class={classes!("border-4", "border-black", "p-4", "shadow-lg")}>
//             <h1 class={classes!("text-custom", "text-4xl", "font-bold")}>
//                 {"Welcome to Mini Note!"}
//             </h1>
//         </div>
//     }
// }

use yew::prelude::*;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <div class="nav-container">
            <h1 class="nav-title">
                {"Welcome to Mini Note!"}
            </h1>
        </div>
    }
}
