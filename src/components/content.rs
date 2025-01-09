use std::cell::RefCell;
use crate::components::script::render_mathjax;
use crate::context::app_context::{AppStateAction, AppStateContext};
use crate::model::outline_tree;
use gloo_net::http::Request;
use outline_tree::TitleNode;
use pulldown_cmark::{Options, Parser};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::string::String;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::Node;
use yew::prelude::*;
use crate::model::outline_tree::OutlineTree;

#[wasm_bindgen]
extern "C" {
    pub fn marked_parse(input: String) -> String;
}


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
        let app_state_ctx = app_state_ctx.clone();

        use_effect(move || {

            let app_state_ctx = app_state_ctx.clone();

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
                    let fetched_markdown = fetched_markdown.replace(r"\\", r"\\\\");

                    // 转换 Markdown 为 HTML
                    let parser = Parser::new_ext(&fetched_markdown, Options::all());
                    let mut html_output = String::new();
                    pulldown_cmark::html::push_html(&mut html_output, parser);

                    // 替换图片路径
                    let processed_html = replace_image_paths(&html_output, &markdown_file_path_clone);

                    // 通过 js 解析
                    // let processed_html = replace_image_paths(&fetched_markdown, &markdown_file_path_clone);
                    // let processed_html = marked_parse(processed_html.to_string());

                    // 从 markdown 里面提取 # 作为大纲
                    let (titles, processed_html) = TitleNode::process_titles_with_ids(&processed_html);

                    if let Some(ctx) = app_state_ctx {
                        ctx.dispatch(AppStateAction::UpdateUserStatus(Some(titles)));
                    }

                    // 更新状态
                    markdown_html.set(processed_html);

                    // 调用 MathJax 渲染数学公式 - 确保只渲染一次
                    render_mathjax();
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
    <div class="markdown-content prose prose-lg max-w-screen-2xl mx-auto prose-p:overflow-x-auto prose-p:overflow-y-hidden">
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

      // 使用 use_effect 在内容渲染后打印 "hello"
                {
                    let markdown_html = markdown_html.clone();
                    use_effect(move || {
                        let window = web_sys::window().unwrap();

                        // 创建一个闭包并将其包装为 js_sys::Function
                        let closure = Closure::<dyn FnMut(f64)>::new(move |_| {
                            web_sys::console::log_1(&JsValue::from_str("hello"));
                        });

                        // 将闭包的所有权绑定到 Rc<RefCell>，确保生命周期延长
                        let closure_rc = Rc::new(RefCell::new(closure));

                        // 调用 request_animation_frame
                        window.request_animation_frame(closure_rc.borrow().as_ref().unchecked_ref()).unwrap();

                        // 保持闭包的生命周期，避免被提前释放
                        let _ = closure_rc; // 延长 closure_rc 的生命周期

                        || {}
                    });
                }

                    Html::VRef(Node::from(div)) // 将 HTML 内容包装为 Yew 的 VNode
                } else {
                    html! { <p>{ "Loading..." }</p> }
                }
            }
        </div>
    }
}

fn remove_last_component(path: &str) -> PathBuf {
    // 替换路径中的反斜杠为正斜杠，确保路径分隔符一致
    let path = path.replace("\\", "/");

    // 将路径字符串转换为 Path 类型
    let path = Path::new(&path);

    // 使用 components() 进行路径分割并去掉最后一个组件
    let parent_path: PathBuf = path.components()
        .take(path.components().count() - 1)  // 去掉最后一部分（文件名或文件夹）
        .collect();

    parent_path
}

// 替换 HTML 中的图片路径
fn replace_image_paths(html: &str, markdown_path: &str) -> String {
    web_sys::console::log_1(&JsValue::from_str(&format!("markdown_path: {}", markdown_path.to_string())));

    // 获取 Markdown 文件的父目录
    let markdown_dir = remove_last_component(markdown_path);

    web_sys::console::log_1(&JsValue::from_str(&format!("markdown_dir: {}", markdown_dir.to_string_lossy().to_string())));


    // 匹配 <img> 标签中的 src 属性
    let img_regex = Regex::new(r#"(?i)<img[^>]*?src="([^"]+)"[^>]*?>"#).unwrap();
    img_regex.replace_all(html, |caps: &regex::Captures| {
        let src = &caps[1];
        // 判断是否为相对路径（不以 http:// 或 https:// 开头）
        if !src.starts_with("http://") && !src.starts_with("https://") {
            // 拼接完整路径
            let full_path = markdown_dir.join(src).to_string_lossy().to_string();

            // web_sys::console::log_1(&JsValue::from_str(&format!("full_path: {}", full_path.to_string())));
            format!(r#"<img src="{}" />"#, full_path)
        } else {
            // 保留原路径
            caps[0].to_string()
        }
    }).to_string()
}
