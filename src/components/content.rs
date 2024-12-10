// 内容展示区域

use yew::prelude::*;

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <div class="px-40 py-10 ">
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
