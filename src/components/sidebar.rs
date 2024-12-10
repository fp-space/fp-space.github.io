use std::collections::HashMap;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct FileNode {
    pub name: String,
    pub is_folder: bool,
    pub children: Option<Vec<FileNode>>,
}

fn load_data() -> Vec<FileNode> {
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

#[function_component(SearchIcon)]
fn search_icon() -> Html {
    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="search-icon"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M15.75 15.75L19.5 19.5M10.5 18.75a8.25 8.25 0 1 1 0-16.5 8.25 8.25 0 0 1 0 16.5z"
            />
        </svg>
    }
}


#[function_component(SearchInput)]
fn search_input() -> Html {
    html! {
        <input
            type="text"
            class="search-input"
            placeholder="查找"
        />
    }
}


#[derive(Properties, PartialEq)]
pub struct FolderIconProps {
    pub used: bool,
}
// 定义 FolderIcon 组件
#[function_component(FolderIcon)]
pub fn folder_icon(props: &FolderIconProps) -> Html {
    // 根据 `used` 状态生成类名
    let class_name = if props.used {
        "folder-icon open"
    } else {
        "folder-icon"
    };

    html! {
        <svg class={class_name} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            {
                if props.used {
                    html! {
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 9l6 6 6-6" /> // 向下箭头
                    }
                } else {
                    html! {
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 6l6 6-6 6" /> // 向右箭头
                    }
                }
            }
        </svg>
    }
}


#[function_component(FileIcon)]
pub fn file_icon() -> Html {
    html! {
        <svg class="file-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
    }
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let file_tree = load_data();

    // 初始化文件夹的展开状态
    let expanded_state = use_state(|| HashMap::<String, bool>::new());

    html! {
        <div class="sidebar">
            <div class="sidebar-tabs">
                <button class="sidebar-tab active">{"文件"}</button>
                <button class="sidebar-tab">{"大纲"}</button>
            </div>
            <div class="sidebar-search">
                <div class="search-container">
                    <SearchInput />
                    <SearchIcon />
                </div>
            </div>
            <div class="sidebar-content">
                {file_tree.iter().map(|node| render_file_node(node, expanded_state.clone())).collect::<Html>()}
            </div>
        </div>
    }
}

fn render_file_node(file_node: &FileNode, expanded_state: UseStateHandle<HashMap<String, bool>>) -> Html {
    let file_name = file_node.name.clone();

    // 获取当前文件夹的展开状态
    let is_expanded = expanded_state
        .get(&file_name)
        .cloned()
        .unwrap_or(false);

    // 点击事件：更新当前文件夹的展开状态
    let toggle = {
        let expanded_state = expanded_state.clone();
        let file_name = file_name.clone();
        Callback::from(move |_| {
            let mut state = (*expanded_state).clone();
            let is_expanded = state.entry(file_name.clone()).or_insert(false);
            *is_expanded = !*is_expanded;
            expanded_state.set(state);
        })
    };

    html! {
        <div class="file-node">
            <div class="file-node-header" onclick={toggle}>
                {
                    if file_node.is_folder {
                        if is_expanded {
                            html! {<FolderIcon used={true} />}
                        } else {

                            html! {<FolderIcon used={false} />}
                        }
                    } else {
                        html! {<FileIcon />}
                    }
                }
                <span class="file-node-name">{ &file_node.name }</span>
            </div>
            {
                if is_expanded && file_node.is_folder {
                    html! {
                        <div class="file-node-children">
                            {file_node.children.as_ref().unwrap_or(&vec![]).iter().map(|child| render_file_node(child, expanded_state.clone())).collect::<Html>()}
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
