use std::collections::HashMap;
use std::path::{PathBuf};
use std::time::SystemTime;

#[derive(Clone, PartialEq, Debug)]
pub struct FileNode {
    pub(crate) name: String,             // 文件或目录的名称
    pub(crate) is_folder: bool,          // 是否是文件夹
    pub(crate) path: PathBuf,            // 文件的完整路径
    pub(crate) size: Option<u64>,             // 文件的大小（字节数）
    pub(crate) created_at: Option<SystemTime>, // 文件或目录的创建时间
    pub(crate) modified_at: Option<SystemTime>, // 文件或目录的最后修改时间
    pub(crate) file_type: Option<String>,      // 文件类型，例如：".txt", ".jpg" 等
    pub(crate) children: Option<HashMap<String, FileNode>>, // 如果是目录，则包含子节点
}
