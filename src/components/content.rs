// 内容展示区域

use yew::prelude::*;
// markdown 渲染库
use yew_markdown::Markdown;

const MARKDOWN_SOURCE: &str =
    include_str!("../../public/storage/StudyNotes/Java编程/07-云原生/01-Docker篇.md");

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <article class="prose lg:prose-xl">
            <Markdown src={MARKDOWN_SOURCE}/>
        </article>
    }
}
