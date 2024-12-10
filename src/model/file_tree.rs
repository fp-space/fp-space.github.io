use std::collections::HashMap;
use std::path::PathBuf;
use crate::model::file_node::FileNode;

#[derive(Clone, PartialEq, Debug)]
pub struct FileTree {
    pub(crate) root: FileNode,                     // 文件树的根节点
    pub(crate) nodes_by_path: HashMap<String, FileNode>, // 用路径作为键，存储所有节点
}

impl FileTree {
    pub(crate) fn new() -> FileTree {
        let mut root_children = HashMap::new();

        // 创建文件夹和文件
        let folder = FileNode {
            name: String::from("Folder 1"),
            is_folder: true,
            path: PathBuf::from("/folder1"),
            size: None,
            created_at: None,
            modified_at: None,
            file_type: None,
            children: Some(HashMap::new()), // 没有子文件夹
        };
        root_children.insert(folder.path.to_string_lossy().to_string(), folder);

        let file = FileNode {
            name: String::from("file1.txt"),
            is_folder: false,
            path: PathBuf::from("/file1.txt"),
            size: Some(1234),
            created_at: None,
            modified_at: None,
            file_type: Some(String::from(".txt")),
            children: None,
        };
        root_children.insert(file.path.to_string_lossy().to_string(), file);

        let root = FileNode {
            name: String::from("Root"),
            is_folder: true,
            path: PathBuf::from("/"),
            size: None,
            created_at: None,
            modified_at: None,
            file_type: None,
            children: Some(root_children),
        };

        let nodes_by_path = HashMap::new();

        FileTree { root, nodes_by_path }
    }
}