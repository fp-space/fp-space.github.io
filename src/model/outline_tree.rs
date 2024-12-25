use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct TitleNode {
    pub level: i8,
    pub title: String,
    pub has_children: bool,  // 用于判断是否有子节点
    pub(crate) children: Vec<Rc<RefCell<TitleNode>>>,  // 存储子节点的 Rc<RefCell>
    pub is_expanded: bool,  // 用于控制节点是否展开
}

impl TitleNode {
    // 创建一个新的节点
    fn new(title: String, level: i8) -> Rc<RefCell<TitleNode>> {
        Rc::new(RefCell::new(TitleNode {
            level,
            title,
            has_children: false,
            children: Vec::new(),
            is_expanded: false,  // 默认节点不展开
        }))
    }

    // 添加子节点
    fn add_child(parent: &Rc<RefCell<TitleNode>>, child: Rc<RefCell<TitleNode>>) {
        parent.borrow_mut().children.push(child);  // 将子节点添加到当前节点的 children 列表中
        parent.borrow_mut().has_children = true;  // 标记当前节点有子节点
    }

    // 设置展开状态
    fn set_expanded(&mut self, expanded: bool) {
        self.is_expanded = expanded;
    }
}

pub trait OutlineTree {
    fn extract_titles_from_markdown(content: &str) -> Vec<Rc<RefCell<TitleNode>>>;
}

impl OutlineTree for TitleNode {

    // 从 Markdown 中提取标题并构建树状结构
    fn extract_titles_from_markdown(content: &str) -> Vec<Rc<RefCell<TitleNode>>> {
        let mut stack: Vec<Rc<RefCell<TitleNode>>> = Vec::new();  // 用于追踪当前路径的栈，存储所有权
        let mut root: Vec<Rc<RefCell<TitleNode>>> = Vec::new();  // 根节点集合

        for line in content.lines() {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with('#') {
                let level = trimmed_line.chars().take_while(|&c| c == '#').count() as i8;  // 计算标题的级别
                let title = trimmed_line.trim_start_matches('#').trim().to_string();  // 获取标题文本

                // 创建一个新的节点
                let new_node = TitleNode::new(title, level);

                // 根据标题级别，决定是否将其添加为根节点或子节点
                if stack.is_empty() {
                    root.push(new_node.clone());  // 如果栈为空，则直接添加为根节点
                } else {
                    // 回退栈中的节点，直到找到合适的父节点
                    while let Some(top) = stack.last() {
                        if top.borrow().level < level {
                            break;  // 找到合适的父节点
                        }
                        stack.pop();  // 弹出比当前级别更高的节点
                    }

                    // 让栈顶节点成为当前标题的父节点
                    if let Some(parent) = stack.last() {
                        TitleNode::add_child(parent, new_node.clone());  // 将当前节点添加为父节点的子节点
                    }
                }

                // 将新的节点加入栈中，转移所有权
                stack.push(new_node);  // 将新的节点加入栈中
            }
        }

        root  // 返回根节点集合
    }
}

use std::io::{self, Read};
use std::fs;

fn main() -> io::Result<()> {
    // 指定要读取的文件路径
    let file_path = "public/storage/StudyNotes/README.md"; // 替换为你的文件路径

    // 读取文件内容并处理可能的错误
    let markdown_content = fs::read_to_string(file_path)?;

    // 从 Markdown 提取标题
    let titles = TitleNode::extract_titles_from_markdown(&markdown_content);

    // 打印大纲
    println!("Outline:");
    println!("{:?}", titles);

    // 返回 Ok(()) 表示没有错误
    Ok(())
}
