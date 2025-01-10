use std::fmt;
use std::fmt::{Display, Formatter};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use std::sync::{Arc, Mutex};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use crate::model::tree::TreeNode;

// 定义文件节点结构体
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize,Eq)]
pub struct FileNode {
    pub name: String,               // 文件或目录名称
    pub is_dir: bool,               // 是否为文件夹
    pub path: Option<String>,       // 文件路径（对于文件夹可以为 None）
    pub file_type: Option<String>,  // 文件类型（如 .md, .txt）
    pub create_time: Option<String>,// 文件创建时间
    pub modify_time: Option<String>,// 文件修改时间
}

impl FileNode {

    // 通过 load_data 函数初始化数据
    pub(crate) async fn load_tree_data() -> Vec<TreeNode<FileNode>>{
        Self::fetch_file_tree("/public/file_tree.json".parse().unwrap()).await.unwrap();
        let result = GLOBAL_FILE_TREE.lock().unwrap().clone();
        vec![result.unwrap()]
    }

    // 异步函数：通过 HTTP 请求获取文件目录信息，并解析 JSON 格式的数据
    async fn fetch_file_tree(url: String) -> Result<(), JsValue> {

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
        let file_tree: TreeNode<FileNode> = from_value(json)?;

        print!("{:?}", file_tree);
        // 将解析后的文件树保存到全局变量中
        let mut file_tree_lock = GLOBAL_FILE_TREE.lock().unwrap();
        *file_tree_lock = Some(file_tree); // 更新全局文件树

        Ok(())
    }


}


impl Display for FileNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Display 文件基本信息
        write!(f, "FileNode {{\n")?;
        write!(f, "  Name: {}\n", self.name)?;
        write!(f, "  Path: {:?}\n", self.path)?;
        write!(f, "  File Type: {:?}\n", self.file_type)?;
        write!(f, "  Created: {:?}\n", self.create_time)?;
        write!(f, "  Modified: {:?}\n", self.modify_time)?;
        write!(f, "  Is Directory: {}\n", self.is_dir)?;

        write!(f, "}}")
    }
}

// 定义一个全局的文件树，用于存储文件的目录结构
pub static GLOBAL_FILE_TREE: Lazy<Arc<Mutex<Option<TreeNode<FileNode>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))  // 先将数据初始化为空，稍后加载 JSON 数据
});


