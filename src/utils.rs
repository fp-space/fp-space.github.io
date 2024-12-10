// 工具函数，可能包括文件加载和异步处理等

use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct FileNode {
    pub name: String,
    pub is_folder: bool,
    pub children: Option<Vec<FileNode>>,
}

pub fn load_data() -> Vec<FileNode> {
    let file_tree = vec![FileNode {
        name: "Yew 篇".to_string(),
        is_folder: true,
        children: Some(vec![
            FileNode {
                name: "环境配置".to_string(),
                is_folder: false,
                children: None,
            },
            FileNode {
                name: "第一个静态页面".to_string(),
                is_folder: false,
                children: None,
            },
            FileNode {
                name: "HTML".to_string(),
                is_folder: true,
                children: Some(vec![
                    FileNode {
                        name: "html! 宏".to_string(),
                        is_folder: false,
                        children: None,
                    },
                    FileNode {
                        name: "事件".to_string(),
                        is_folder: true,
                        children: Some(vec![
                            FileNode {
                                name: "事件监听器".to_string(),
                                is_folder: false,
                                children: None,
                            },
                            FileNode {
                                name: "事件冒泡".to_string(),
                                is_folder: false,
                                children: None,
                            },
                        ]),
                    },
                ]),
            },
        ]),
    }];
    file_tree
}
