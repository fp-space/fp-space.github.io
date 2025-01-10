use std::cell::RefCell;
use std::rc::Rc;
use regex::Regex;
use crate::model::tree::{TreeNode, RuntimeTreeNode};

#[derive(Debug, Clone, PartialEq)]
pub struct TitleNode {
    pub level: usize,
    pub title: String,
}

impl TitleNode {
    fn new(title: String, level: usize) -> TitleNode {
        TitleNode { level, title }
    }
}

pub trait OutlineTree {
    fn process_titles_with_ids(content: &str) -> (Vec<TreeNode<TitleNode>>, String);
}

pub fn sanitize_title(title: &str) -> String {
    title.to_string()
}

impl OutlineTree for TitleNode {
    fn process_titles_with_ids(content: &str) -> (Vec<TreeNode<TitleNode>>, String) {
        // 使用 `RuntimeTreeNode<TitleNode>` 作为运行时管理结构
        let mut stack: Vec<Rc<RefCell<RuntimeTreeNode<TitleNode>>>> = Vec::new();
        let mut root: Vec<Rc<RefCell<RuntimeTreeNode<TitleNode>>>> = Vec::new();
        let mut updated_content = String::new();

        let re = Regex::new(r"<h([1-6])>(.*?)</h([1-6])>").expect("Invalid regex pattern");

        for line in content.lines() {
            if let Some(captures) = re.captures(line) {
                let level: usize = captures.get(1).unwrap().as_str().parse().expect("Invalid level");
                let title = captures.get(2).unwrap().as_str().to_string();

                println!("Level: {}, Title: {}", level, title.clone());

                // 生成 ID 并添加到标签中
                let id = sanitize_title(&title);
                let updated_line = format!("<h{level} id=\"{id}\">{title}</h{level}>");
                updated_content.push_str(&updated_line);
                updated_content.push('\n');

                // 创建一个新的 `RuntimeTreeNode<TitleNode>`
                let new_node = Rc::new(RefCell::new(RuntimeTreeNode {
                    data: TitleNode::new(title, level),
                    children: Vec::new(),
                    is_expanded: false,
                    has_children: false,
                }));

                // 如果栈为空，直接作为根节点添加
                if stack.is_empty() {
                    root.push(new_node.clone());
                } else {
                    // 回溯栈，找到合适的父节点
                    while let Some(top) = stack.last() {
                        if top.borrow().data.level < level {
                            break; // 找到合适的父节点
                        }
                        stack.pop(); // 弹出比当前级别更高的节点
                    }

                    // 如果找到父节点，将新节点作为子节点添加
                    if let Some(parent) = stack.last() {
                        RuntimeTreeNode::add_child_ref(parent, new_node.clone());
                    } else {
                        root.push(new_node.clone()); // 如果没有父节点，直接作为根节点
                    }
                }

                // 将当前节点加入栈
                stack.push(new_node.clone());
            } else {
                // 如果不是标题行，直接加到更新后的内容
                updated_content.push_str(line);
                updated_content.push('\n');
            }
        }

        // 将运行时结构转换为普通结构以支持序列化
        let converted_root: Vec<TreeNode<TitleNode>> = root
            .into_iter()
            .map(|node| RuntimeTreeNode::to_tree_node(&node))
            .collect();

        (converted_root, updated_content)
    }
}
