// use wasm_bindgen::prelude::*; // 引入 wasm_bindgen 库，用于与 JavaScript 交互
// use web_sys::{Request, RequestInit, Response}; // 引入 web_sys 库中的 Request 和 Response 类型，用于网络请求
use serde::{Deserialize, Serialize}; // 引入 serde 库，用于 JSON 序列化与反序列化
// use serde_wasm_bindgen::from_value; // 引入 serde_wasm_bindgen 库，用于将 JavaScript 值转为 Rust 类型
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

// use once_cell::sync::Lazy;
// use serde_wasm_bindgen::from_value;
// use wasm_bindgen::{JsCast, JsValue};
// use wasm_bindgen::prelude::wasm_bindgen;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::{Request, RequestInit, Response};
//
// // use gloo_net::websocket::State::Open;
// // // 引入 Rust 的同步原语，用于共享状态
// // use once_cell::sync::Lazy;
// // // 引入 once_cell 库，用于实现延迟初始化
// // use wasm_bindgen_futures::JsFuture; // 引入 wasm_bindgen_futures 库，用于处理异步操作
// // use wasm_bindgen::JsCast; // 引入 wasm_bindgen 的 JsCast 功能，用于类型转换
// // //
// // 定义文件节点结构体
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct FileNode {
    pub name: String,               // 文件或目录名称
    pub is_dir: bool,               // 是否为文件夹
    pub children: Option<Vec<FileNode>>, // 子目录，若为文件，则为 None
    pub path: String,               // 文件路径
    pub file_type: String,          // 文件类型（如 .md, .txt）
    pub create_time: String,        // 文件创建时间
    pub modify_time: String,        // 文件修改时间
}


// 通过 load_data 函数初始化数据
pub async fn load_tree_data() -> Vec<FileNode> {
    fetch_file_tree("http://127.0.0.1:8341/public/file_tree.json".parse().unwrap()).await.unwrap();
    let result = GLOBAL_FILE_TREE.lock().unwrap().clone();
    vec![result.unwrap()]
}

// 定义一个全局的文件树，用于存储文件的目录结构
pub static GLOBAL_FILE_TREE: Lazy<Arc<Mutex<Option<FileNode>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))  // 先将数据初始化为空，稍后加载 JSON 数据
});

// 异步函数：通过 HTTP 请求获取文件目录信息，并解析 JSON 格式的数据
#[wasm_bindgen]
pub async fn fetch_file_tree(url: String) -> Result<(), JsValue> {

    println!("开始");

    // 创建请求对象
    let opts = RequestInit::new();
    opts.set_method("GET"); // 设置请求方法为 GET
    opts.set_mode(web_sys::RequestMode::Cors); // 设置请求模式为 CORS

    // 创建一个新的请求
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let response = JsFuture::from(window.fetch_with_request(&request)).await?; // 异步发起请求并获取响应

    // 检查响应是否有效
    let response: Response = response.dyn_into()?;
    if !response.ok() {
        return Err(JsValue::from_str("Failed to fetch file tree"));
    }

    // 获取响应体并将其转为 JSON 数据
    let json = JsFuture::from(response.json()?).await?;
    let file_tree: FileNode = from_value(json)?;

    // 将解析后的文件树保存到全局变量中
    let mut file_tree_lock = GLOBAL_FILE_TREE.lock().unwrap();
    *file_tree_lock = Some(file_tree); // 更新全局文件树

    Ok(())
}