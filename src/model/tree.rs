// src/model/tree.rs

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

/// 通用的树节点结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TreeNode<T> {
    pub data: T,
    pub children: Option<Vec<TreeNode<T>>>,
    #[serde(default)]
    pub is_expanded: bool,
    #[serde(default)]
    pub has_children: bool,
}

impl<T> TreeNode<T> {
    /// 创建一个新地树节点
    pub fn new(data: T) -> Self {
        TreeNode {
            data,
            children: None, // 初始化为 None
            is_expanded: false,
            has_children: false,
        }
    }

    pub fn add_child(&mut self, node: TreeNode<T>) {
        match &mut self.children {
            Some(children) => children.push(node),
            None => self.children = Some(vec![node]),
        }
        self.is_expanded = true;
        self.has_children = true;
    }
}