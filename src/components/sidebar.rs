use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct FileNode {
    pub name: String,
    pub is_folder: bool,
    pub children: Option<Vec<FileNode>>,
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let file_tree = vec![
        FileNode {
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
        },
    ];

    let is_expanded = use_state(|| true);

    html! {
        <div class="sidebar">
            <div class="sidebar-tabs">
                <button class="sidebar-tab active">{"文件"}</button>
                <button class="sidebar-tab">{"大纲"}</button>
            </div>
            <div class="sidebar-search">
                <div class="search-container">
                    <input
                        type="text"
                        class="search-input"
                        placeholder="查找"
                    />
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="search-icon"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.75 15.75L19.5 19.5M10.5 18.75a8.25 8.25 0 1 1 0-16.5 8.25 8.25 0 0 1 0 16.5z" />
                    </svg>
                </div>
            </div>
            <div class="sidebar-content">
                {file_tree.iter().map(|node| render_file_node(node, is_expanded.clone())).collect::<Html>()}
            </div>
        </div>
    }
}

fn render_file_node(file_node: &FileNode, is_expanded: UseStateHandle<bool>) -> Html {
    let toggle = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| is_expanded.set(!*is_expanded))
    };

    html! {
        <div class="file-node">
            <div class="file-node-header" onclick={toggle}>
                {
                    if file_node.is_folder {
                        if *is_expanded {
                            html! {
                                <svg class="folder-icon open" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 9l6 6 6-6" />
                                </svg>
                            }
                        } else {
                            html! {
                                <svg class="folder-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 6l6 6-6 6" />
                                </svg>
                            }
                        }
                    } else {
                        html! {
                            <svg class="file-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                            </svg>
                        }
                    }
                }
                <span class="file-node-name">{ &file_node.name }</span>
            </div>
            {
                if *is_expanded && file_node.is_folder {
                    html! {
                        <div class="file-node-children">
                            {file_node.children.as_ref().unwrap_or(&vec![]).iter().map(|child| render_file_node(child, is_expanded.clone())).collect::<Html>()}
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
