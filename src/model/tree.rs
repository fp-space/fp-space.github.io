use std::cell::RefCell;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

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

impl<T> TreeNode<T> where T: Clone{
    /// 创建一个新地树节点
    pub fn new(data: T) -> Self {
        TreeNode {
            data,
            children: None, // 初始化为 None
            is_expanded: false,
            has_children: false,
        }
    }

    #[warn(dead_code)]
    pub fn add_child(&mut self, node: TreeNode<T>) {
        match &mut self.children {
            Some(children) => children.push(node),
            None => self.children = Some(vec![node]),
        }
        self.is_expanded = true;
        self.has_children = true;
    }
}

// 运行时专用的树节点结构
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeTreeNode<T> {
    pub data: T,
    pub children: Vec<Rc<RefCell<RuntimeTreeNode<T>>>>,
    pub is_expanded: bool,
    pub has_children: bool,
}

impl<T: Clone> RuntimeTreeNode<T> {
    /// 从普通的 TreeNode 转换为 RuntimeTreeNode
    pub fn from_tree_node(node: &TreeNode<T>) -> Rc<RefCell<Self>> {
        let children = match &node.children {
            Some(child_nodes) => child_nodes
                .iter()
                .map(|child| Self::from_tree_node(child))
                .collect(),
            None => Vec::new(),
        };

        Rc::new(RefCell::new(RuntimeTreeNode {
            data: node.data.clone(),
            children,
            is_expanded: false,
            has_children: false,
        }))
    }

    /// 将 RuntimeTreeNode 转换为普通的 TreeNode
    pub fn to_tree_node(node: &Rc<RefCell<Self>>) -> TreeNode<T> {
        let node = node.borrow();
        let children = if node.children.is_empty() {
            None
        } else {
            Some(
                node.children
                    .iter()
                    .map(|child| Self::to_tree_node(child))
                    .collect(),
            )
        };

        TreeNode {
            data: node.data.clone(),
            children,
            is_expanded: false, // 根据需要设置
            has_children: !node.children.is_empty(),
        }
    }

    pub fn add_child_ref(parent: &Rc<RefCell<RuntimeTreeNode<T>>>, child: Rc<RefCell<RuntimeTreeNode<T>>>) {
        let mut parent_mut = parent.borrow_mut(); // 获取父节点的可变引用
        parent_mut.children.push(child); // 将子节点添加到 `children` 列表
        parent_mut.has_children = true; // 标记父节点有子节点
    }

}