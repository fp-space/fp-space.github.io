use std::string::String;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use crate::context::app_context::AppStateContext;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use pulldown_cmark::{Parser, Options};
use web_sys::Node;
use regex::Regex;
use std::path::Path;

#[function_component(MainContent)]
pub fn main_content() -> Html {

    // 监听 AppStateContext 的变化
    let app_state_ctx = use_context::<AppStateContext>();
    let markdown_html = use_state(|| String::new());
    let markdown_file_path = use_state(|| String::new()); // 使用 String 代替 Option<String>
    let last_markdown_file_path = use_state(|| String::new()); // 使用 String 代替 Option<String>

    {
        let markdown_html = markdown_html.clone(); // 克隆 UseStateHandle
        let markdown_file_path = markdown_file_path.clone();
        let last_markdown_file_path = last_markdown_file_path.clone();
        use_effect(move || {
            // 只有当 markdown_file_path 不为空并且与 last_markdown_file_path 不同时才触发请求
            if !(*markdown_file_path).is_empty() && *last_markdown_file_path != *markdown_file_path {
                let markdown_file_path_clone = (*markdown_file_path).clone(); // 在闭包外克隆
                spawn_local(async move {
                    let fetched_markdown = Request::get(&markdown_file_path_clone) // 使用克隆的路径
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                    // 转换 Markdown 为 HTML
                    let parser = Parser::new_ext(&fetched_markdown, Options::all());
                    let mut html_output = String::new();
                    pulldown_cmark::html::push_html(&mut html_output, parser);

                    // 替换图片路径
                    let processed_html = replace_image_paths(&html_output, &markdown_file_path_clone);

                    // 更新状态
                    markdown_html.set(processed_html);
                });
                last_markdown_file_path.set((*markdown_file_path).clone()); // 更新 last_markdown_file_path
            }
            // 返回一个清理函数（组件卸载时触发）
            || {}
        });
    }

    // 更新 markdown_file_path
    if let Some(app_state_ctx) = app_state_ctx {
        if let Some(ref file_node) = app_state_ctx.selected_file {
            // 如果文件路径变化才更新
            if *markdown_file_path != file_node.path {
                markdown_file_path.set(file_node.path.clone()); // 更新文件路径
                web_sys::console::log_1(&JsValue::from_str(&format!("Selected File: {}", file_node.to_string())));
            }
        }
    }

    html! {
    <div class="markdown-content prose prose-lg max-w-none">
            {
                // 使用 .get() 方法借用 markdown_html 的值
                if !(*markdown_html).is_empty() {
                    let div = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .create_element("div")
                        .unwrap();
                    div.set_inner_html(&*markdown_html); // 使用借用的值
                    Html::VRef(Node::from(div)) // 将 HTML 内容包装为 Yew 的 VNode
                } else {
                    html! { <p>{ "Loading..." }</p> }
                }
            }
        </div>
    }
}

// 替换 HTML 中的图片路径
fn replace_image_paths(html: &str, markdown_path: &str) -> String {
    // 获取 Markdown 文件的父目录
    let markdown_dir = Path::new(markdown_path)
        .parent()
        .unwrap_or_else(|| Path::new(""));

    // 匹配 <img> 标签中的 src 属性
    let img_regex = Regex::new(r#"(?i)<img[^>]*?src="([^"]+)"[^>]*?>"#).unwrap();
    img_regex.replace_all(html, |caps: &regex::Captures| {
        let src = &caps[1];
        // 判断是否为相对路径（不以 http:// 或 https:// 开头）
        if !src.starts_with("http://") && !src.starts_with("https://") {
            // 拼接完整路径
            let full_path = markdown_dir.join(src).to_string_lossy().to_string();
            format!(r#"<img src="{}" />"#, full_path)
        } else {
            // 保留原路径
            caps[0].to_string()
        }
    }).to_string()
}
