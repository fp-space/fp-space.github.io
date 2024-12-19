use std::fmt;
use std::fmt::{Display, Formatter};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use std::sync::{Arc, Mutex};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

// 定义文件节点结构体
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize,Eq)]
pub struct FileNode {
    pub name: String,               // 文件或目录名称
    pub is_dir: bool,               // 是否为文件夹
    pub children: Option<Vec<FileNode>>, // 子目录，若为文件，则为 None
    pub path: String,               // 文件路径
    pub file_type: String,          // 文件类型（如 .md, .txt）
    pub create_time: String,        // 文件创建时间
    pub modify_time: String,        // 文件修改时间
    #[serde(default)]
    pub is_expanded: bool,          // 是否展开
}

pub trait FileTree{
    fn new(name: String,
           is_dir: bool,
           children: Option<Vec<FileNode>>,
           path: String,
           file_type: String,
           create_time: String,
           modify_time: String, ) -> Self; // 切换文件夹的展开状态
    fn toggle_expanded(&mut self); // 获取当前文件节点的展开状态
    fn is_expanded(&self) -> bool; // 获取展开后的子节点（如果是目录并且展开，返回子节点，否则返回 None）
    fn get_expanded_children(&self) -> Option<Vec<FileNode>>; // 根据给定的路径查找特定的文件节点
    fn find_node_by_path(&self, path: &str) -> Option<&FileNode>; // 递归地获取所有的子节点路径
    fn get_all_paths(&self) -> Vec<String>; // 通过 load_data 函数初始化数据
    async fn load_tree_data() -> Vec<FileNode>; // 异步函数：通过 HTTP 请求获取文件目录信息，并解析 JSON 格式的数据
    // #[wasm_bindgen]
    async fn fetch_file_tree(url: String) -> Result<(), JsValue>;
}

impl FileNode {
    // 构造一个新的文件节点
    fn new(
        name: String,
        is_dir: bool,
        children: Option<Vec<FileNode>>,
        path: String,
        file_type: String,
        create_time: String,
        modify_time: String,
    ) -> Self {
        FileNode {
            name,
            is_dir,
            children,
            path,
            file_type,
            create_time,
            modify_time,
            is_expanded: false, // 默认展开状态为 false
        }
    }

    // 切换文件夹的展开状态
    fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    // 获取当前文件节点的展开状态
    fn is_expanded(&self) -> bool {
        self.is_expanded
    }

    // 获取展开后的子节点（如果是目录并且展开，返回子节点，否则返回 None）
    fn get_expanded_children(&self) -> Option<Vec<FileNode>> {
        if self.is_expanded {
            self.children.clone()
        } else {
            None
        }
    }

    // 根据给定的路径查找特定的文件节点
    fn find_node_by_path(&self, path: &str) -> Option<&FileNode> {
        if self.path == path {
            return Some(self);
        }

        // 如果当前节点有子节点，则递归查找
        if let Some(children) = &self.children {
            for child in children {
                if let Some(found) = child.find_node_by_path(path) {
                    return Some(found);
                }
            }
        }

        None
    }

    // 递归地获取所有的子节点路径
    fn get_all_paths(&self) -> Vec<String> {
        let mut paths = vec![self.path.clone()];
        if let Some(children) = &self.children {
            for child in children {
                paths.extend(child.get_all_paths());
            }
        }
        paths
    }


    // 通过 load_data 函数初始化数据
    pub(crate) async fn load_tree_data() -> Vec<FileNode>{
        Self::fetch_file_tree("/public/file_tree.json".parse().unwrap()).await.unwrap();
        let result = GLOBAL_FILE_TREE.lock().unwrap().clone();
        vec![result.unwrap()]
    }


    // 异步函数：通过 HTTP 请求获取文件目录信息，并解析 JSON 格式的数据
    // #[wasm_bindgen]
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
        let file_tree: FileNode = from_value(json)?;

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
        write!(f, "  Path: {}\n", self.path)?;
        write!(f, "  File Type: {}\n", self.file_type)?;
        write!(f, "  Created: {}\n", self.create_time)?;
        write!(f, "  Modified: {}\n", self.modify_time)?;
        write!(f, "  Is Directory: {}\n", self.is_dir)?;
        write!(f, "  Is Expanded: {}\n", self.is_expanded)?;

        // 如果是目录，并且有子节点，递归展示子目录
        if let Some(children) = &self.children {
            write!(f, "  Children:\n")?;
            for child in children {
                write!(f, "    {}\n", child)?;
            }
        } else {
            write!(f, "  No children\n")?;
        }

        write!(f, "}}")
    }
}

// 定义一个全局的文件树，用于存储文件的目录结构
pub static GLOBAL_FILE_TREE: Lazy<Arc<Mutex<Option<FileNode>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))  // 先将数据初始化为空，稍后加载 JSON 数据
});


