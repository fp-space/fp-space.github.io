use crate::components::icon::{FileIcon, FolderIcon, SearchIcon};
use std::collections::HashMap;
use yew::prelude::*;
use crate::utils::{load_data, FileNode};

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

#[derive(PartialEq)]
enum Tab {
    File,
    Outline,
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let active_tab = use_state(|| Tab::File);

    // 切换到文件视图
    let show_file = {
        let active_tab = active_tab.clone();
        Callback::from(move |_: MouseEvent| {
            active_tab.set(Tab::File); // 切换视图
        })
    };

    // 切换到大纲视图
    let show_outline = {
        let active_tab = active_tab.clone();
        Callback::from(move |_: MouseEvent| {
            active_tab.set(Tab::Outline); // 切换视图
        })
    };

    html! {
        <div class="sidebar">
            <div class="sidebar-tabs">
                <button
                    class={if *active_tab == Tab::File { "sidebar-tab active" } else { "sidebar-tab" }}
                    onclick={show_file}
                >
                    { "文件" }
                </button>
                <button
                    class={if *active_tab == Tab::Outline { "sidebar-tab active" } else { "sidebar-tab" }}
                    onclick={show_outline}
                >
                    { "大纲" }
                </button>
            </div>
            <div class="sidebar-view">
                {
                    if *active_tab == Tab::File {
                        html! { <FileView /> }
                    } else {
                        html! { <OutlineView /> }
                    }
                }
            </div>
        </div>
    }
}

// 大纲组件
#[function_component(OutlineView)]
fn outline_view() -> Html {
    let file_tree = load_data();

    // 初始化文件夹的展开状态
    let expanded_state = use_state(|| HashMap::<String, bool>::new());

    html! {
        <div class="sidebar-content">
            <div class="sidebar-search">
                <div class="search-container">
                    <SearchInput />
                    <SearchIcon />
                </div>
            </div>
            <div class="outline-tree">
                {file_tree.iter().map(|node| render_file_node(node, expanded_state.clone())).collect::<Html>()}
            </div>
        </div>
    }
}

// 文件组件
#[function_component(FileView)]
fn file_view() -> Html {
    let file_tree = load_data();

    // 初始化文件夹的展开状态
    let expanded_state = use_state(|| HashMap::<String, bool>::new());

    html! {
        <div class="sidebar-content">
            <div class="sidebar-search">
                <div class="search-container">
                    <SearchInput />
                    <SearchIcon />
                </div>
            </div>
            <div class="file-tree">
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
            // 节点头部（文件夹或文件）
            <div class="file-node-header" onclick={toggle}>
                {
                    if file_node.is_folder {
                        html! { <FolderIcon used={is_expanded} /> }
                    } else {
                        html! { <FileIcon /> }
                    }
                }
                <span class="file-node-name">{ &file_node.name }</span>
            </div>

            // 子节点（仅在文件夹展开时渲染）
            {
                if is_expanded && file_node.is_folder {
                    html! {
                        <div class="file-node-children">
                            {
                                file_node.children.as_ref()
                                    .unwrap_or(&vec![])
                                    .iter()
                                    .map(|child| render_file_node(child, expanded_state.clone()))
                                    .collect::<Html>()
                            }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
