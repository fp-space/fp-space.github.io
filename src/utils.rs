// 工具函数，可能包括文件加载和异步处理等

use crate::model::file_tree::FileNode;
pub fn load_data() -> Vec<FileNode> {
    let file_tree = vec![FileNode {
        name: "Yew 篇".to_string(),
        is_dir: true,
        file_type: "folder".to_string(),
        create_time: "2024-01-01 10:00:00".to_string(),
        modify_time: "2024-01-05 15:30:00".to_string(),
        children: Some(vec![
            FileNode {
                name: "环境配置".to_string(),
                is_dir: false,
                file_type: ".md".to_string(),
                create_time: "2024-01-02 09:00:00".to_string(),
                modify_time: "2024-01-03 14:00:00".to_string(),
                children: None,
                path: "".to_string(),
            },
            FileNode {
                name: "第一个静态页面".to_string(),
                is_dir: false,
                file_type: ".md".to_string(),
                create_time: "2024-01-02 10:30:00".to_string(),
                modify_time: "2024-01-03 16:00:00".to_string(),
                children: None,
                path: "".to_string(),
            },
            FileNode {
                name: "HTML".to_string(),
                is_dir: true,
                file_type: "folder".to_string(),
                create_time: "2024-01-01 11:00:00".to_string(),
                modify_time: "2024-01-04 12:30:00".to_string(),
                children: Some(vec![
                    FileNode {
                        name: "html! 宏".to_string(),
                        is_dir: false,
                        file_type: ".md".to_string(),
                        create_time: "2024-01-03 08:30:00".to_string(),
                        modify_time: "2024-01-04 11:00:00".to_string(),
                        children: None,
                        path: "".to_string(),
                    },
                    FileNode {
                        name: "事件".to_string(),
                        is_dir: true,
                        file_type: "folder".to_string(),
                        create_time: "2024-01-02 14:00:00".to_string(),
                        modify_time: "2024-01-05 10:00:00".to_string(),
                        children: Some(vec![
                            FileNode {
                                name: "事件监听器".to_string(),
                                is_dir: false,
                                file_type: ".md".to_string(),
                                create_time: "2024-01-03 10:00:00".to_string(),
                                modify_time: "2024-01-04 10:30:00".to_string(),
                                children: None,
                                path: "".to_string(),
                            },
                            FileNode {
                                name: "事件冒泡".to_string(),
                                is_dir: false,
                                file_type: ".md".to_string(),
                                create_time: "2024-01-03 15:00:00".to_string(),
                                modify_time: "2024-01-05 09:00:00".to_string(),
                                children: None,
                                path: "".to_string(),
                            },
                        ]),
                        path: "".to_string(),
                    },
                ]),
                path: "".to_string(),
            },
        ]),
        path: "".to_string(),
    }];
    file_tree
}




pub(crate) fn load_markdown_files(query: String) -> Vec<(String, usize, Vec<(usize, String)>)> {
    let markdown_files = vec![
        // 假设这里是文件名和内容的键值对
        ("file1.md".to_string(), "This is a sample file.\nAnother line.".to_string()),
        ("file2.md".to_string(), "This file contains Rust code.\nA Rust example.".to_string()),
    ];

    if query.is_empty() {
        return Vec::new(); // 如果查询为空，返回空结果
    }

    markdown_files
        .iter()
        .filter(|(_, content)| content.to_lowercase().contains(&query.to_lowercase())) // 筛选包含关键词的文件
        .map(|(name, content)| {
            let matched_lines: Vec<(usize, String)> = content
                .lines()
                .enumerate() // 获取每行的行号
                .filter(|(_, line)| line.to_lowercase().contains(&query.to_lowercase())) // 筛选包含关键词的行
                .map(|(index, line)| (index + 1, line.to_string())) // 返回行号和行内容
                .collect();

            let match_count = matched_lines.len(); // 总匹配条数
            (name.clone(), match_count, matched_lines) // 返回文件名、匹配总数、匹配内容
        })
        .collect()
}
