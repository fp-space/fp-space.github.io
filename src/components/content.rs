// 内容展示区域

use yew::prelude::*;
// markdown 渲染库
use yew_markdown::Markdown;
// 图片的路径示范http://localhost.:8341/public/storage/StudyNotes/Java%E7%BC%96%E7%A8%8B/07-%E4%BA%91%E5%8E%9F%E7%94%9F/assets/01-Docker%E7%AF%87/image-20230413140446753.png
// 需要将项目的问题弄成这样的路径才行
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
