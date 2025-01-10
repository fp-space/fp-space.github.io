// src/model/tree.rs

use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::{MapAccess, Visitor};
use crate::model::file_tree::FileNode;

/// 通用的树节点结构
#[derive(Debug, Clone, PartialEq, Serialize)]
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


// 自定义反序列化实现
impl<'de> Deserialize<'de> for TreeNode<FileNode> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TreeNodeVisitor;

        impl<'de> Visitor<'de> for TreeNodeVisitor {
            type Value = TreeNode<FileNode>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct TreeNode")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut is_dir = None;
                let mut children = None;
                let mut path = None;
                let mut file_type = None;
                let mut create_time = None;
                let mut modify_time = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "is_dir" => {
                            is_dir = Some(map.next_value()?);
                        }
                        "children" => {
                            children = Some(map.next_value()?);
                        }
                        "path" => {
                            path = Some(map.next_value()?);
                        }
                        "file_type" => {
                            file_type = Some(map.next_value()?);
                        }
                        "create_time" => {
                            create_time = Some(map.next_value()?);
                        }
                        "modify_time" => {
                            modify_time = Some(map.next_value()?);
                        }
                        _ => {}
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let is_dir = is_dir.ok_or_else(|| de::Error::missing_field("is_dir"))?;
                let children = children.unwrap_or_else(|| Some(Vec::new()));

                let data = FileNode {
                    name,
                    is_dir,
                    path,
                    file_type,
                    create_time,
                    modify_time,
                };

                let has_children = children.clone().is_none();
                Ok(TreeNode {
                    data,
                    children,
                    is_expanded: false,
                    has_children,
                })
            }
        }

        deserializer.deserialize_map(TreeNodeVisitor)
    }
}