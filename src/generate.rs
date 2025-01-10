
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs::{self, File}, path::Path};
use crate::model::file_tree::FileNode;
use crate::model::tree::TreeNode;

mod model;

// 获取文件的创建时间或修改时间
fn get_metadata_time(path: &Path, is_creation: bool) -> String {
    let metadata = path.metadata().unwrap();
    let time = if is_creation {
        metadata.created().unwrap_or_else(|_| metadata.modified().unwrap())
    } else {
        metadata.modified().unwrap()
    };
    let datetime: DateTime<Local> = DateTime::from(time);
    datetime.to_rfc3339()
}

// 递归生成文件树
fn generate_file_tree(dir: &str, exclude_dirs: Vec<&str>, include_files: Vec<&str>) -> TreeNode<FileNode> {
    let root_path = Path::new(dir);
    let root_name = root_path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("root")).to_string_lossy().to_string();

    let mut root_node = TreeNode::<FileNode>::new(FileNode {
        name: root_name,
        is_dir: true,
        path: Some(root_path.to_string_lossy().to_string()),
        file_type: Some("folder".to_string()),
        create_time: Some(get_metadata_time(&root_path, true)),
        modify_time: Some(get_metadata_time(&root_path, false)),
    });

    // 递归遍历目录
    for entry in fs::read_dir(root_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let is_dir = entry.file_type().unwrap().is_dir();
        let is_file = entry.file_type().unwrap().is_file();

        // 排序目录
        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            // 如果路径是一个目录且被排除，则跳过
            if is_dir && (exclude_dirs.iter().any(|&dir| file_name == dir) || file_name.starts_with(".")) {
                continue;
            }
        }


        // 如果是文件且符合条件，则加入到文件树中
        if is_file && is_valid_file(&path, &include_files) {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let file_extension = path.extension().unwrap_or_default().to_string_lossy().to_string();

            let file_node = FileNode {
                name: file_name.clone(),
                is_dir: false,
                path: Some(path.as_path().to_string_lossy().to_string()),
                file_type: Some(file_extension),
                create_time: Some(get_metadata_time(&path, true)),
                modify_time: Some(get_metadata_time(&path, false)),
            };

            root_node.add_child(TreeNode::new(file_node));
        }

        // 如果是目录，则递归处理子目录
        if is_dir {
            let subdir_node = generate_file_tree(path.to_str().unwrap(), exclude_dirs.clone(), include_files.clone());
            root_node.add_child(subdir_node);
        }
    }

    root_node
}

// 判断文件是否符合指定的扩展名
fn is_valid_file(path: &Path, include_files: &Vec<&str>) -> bool {
    if let Some(extension) = path.extension() {
        let extension_str = extension.to_string_lossy().to_lowercase();
        return include_files.iter().any(|&ext| extension_str == ext.to_lowercase());
    }
    false
}

// 将文件树写入 JSON 文件
fn write_file_tree_to_json(file_tree: &TreeNode<FileNode>, output_path: &str) {
    let file = File::create(output_path).unwrap();
    serde_json::to_writer_pretty(file, &file_tree).unwrap();
    println!("File tree has been written to {}", output_path);
}

fn main() {
    let dir = "public/storage/StudyNotes"; // 指定根目录
    let exclude_dirs = vec!["node_modules", ".git", "target", "assets"]; // 排除的目录
    let include_files = vec!["md", "txt", "json"]; // 包含的文件类型

    // 生成文件树
    let file_tree = generate_file_tree(dir, exclude_dirs, include_files);

    // 将文件树写入 JSON 文件
    write_file_tree_to_json(&file_tree, "public/file_tree.json");
}
