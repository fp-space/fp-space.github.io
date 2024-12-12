use crate::components::icon::{FileIcon, FolderIcon};
use crate::model::file_tree::{load_tree_data, FileNode};
use crate::utils::{load_data, load_markdown_files};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(SearchInput)]
fn search_input() -> Html {
    let search_query = use_context::<UseStateHandle<String>>().expect("No search_query context found");

    let on_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                search_query.set(input.value());
            }
        })
    };

    html! {
        <input
            type="text"
            class="search-input"
            placeholder="查找"
            oninput={on_input}
        />
    }
}



#[derive(PartialEq)]
enum Tab {
    File,
    Outline,
    Search
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let active_tab = use_state(|| Tab::File);
    let search_query = use_state(|| String::new()); // 搜索关键词

    // 切换到文件视图
    let show_file = {
        let active_tab = active_tab.clone();
        Callback::from(move |_: MouseEvent| {
            active_tab.set(Tab::File);
        })
    };

    // 切换到大纲视图
    let show_outline = {
        let active_tab = active_tab.clone();
        Callback::from(move |_: MouseEvent| {
            active_tab.set(Tab::Outline);
        })
    };

    // 搜索状态管理
    let on_search = {
        let active_tab = active_tab.clone();
        let _search_query = search_query.clone();
        Callback::from(move |query: String| {
            if !query.is_empty() {
                active_tab.set(Tab::Search);
            } else {
                active_tab.set(Tab::File);
            }
        })
    };

    {
        let search_query = search_query.clone();
        let on_search = on_search.clone();

        use_effect_with((*search_query).clone(), move |query| {
            on_search.emit(query.clone()); // 显式触发 Callback
            || ()
        });
    }

    html! {
        <ContextProvider<UseStateHandle<String>> context={search_query.clone()}>
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

                    <div class="sidebar-search">
                        <SearchInput />
                    </div>

                    <div class="sidebar-view">
                        <div class={if *active_tab == Tab::File { "tab-content active" } else { "tab-content" }}>
                            <FileView />
                        </div>
                        <div class={if *active_tab == Tab::Outline { "tab-content active" } else { "tab-content" }}>
                            <OutlineView />
                        </div>
                        <div class={if *active_tab == Tab::Search { "tab-content active" } else { "tab-content" }}>
                            <SearchView />
                        </div>
                    </div>
            </div>
        </ContextProvider<UseStateHandle<String>>>
    }
}

#[function_component(SearchView)]
fn search_view() -> Html {
    let search_query = use_context::<UseStateHandle<String>>().unwrap(); // 使用 Context
    let query = (*search_query).to_lowercase();

    // 调用 `load_markdown_files` 获取筛选后的数据
    let results = load_markdown_files(query.clone());

    // 状态：存储每个文件是否展开
    let expanded_state = use_state(|| HashMap::<String, bool>::new());

    html! {
        <div class="search-results">
            {
                if query.is_empty() {
                    html! { <p>{ "请输入搜索关键词..." }</p> }
                } else if results.is_empty() {
                    html! { <p>{ "未找到相关内容" }</p> }
                } else {
                    html! {
                        <ul class="search-results-list">
                            { results.iter().map(|(name, match_count, lines)| {
                                // 是否展开的状态
                                let is_expanded = expanded_state
                                    .get(name)
                                    .cloned()
                                    .unwrap_or(false);

                                // 点击事件：展开或收起
                                let toggle_expand = {
                                    let expanded_state = expanded_state.clone();
                                    let file_name = name.clone();
                                    Callback::from(move |_| {
                                        let mut state = (*expanded_state).clone();
                                        let is_expanded = state.entry(file_name.clone()).or_insert(false);
                                        *is_expanded = !*is_expanded;
                                        expanded_state.set(state);
                                    })
                                };

                                html! {
                                    <li class="search-result-item">
                                        <div class="result-header" onclick={toggle_expand}>
                                            <span class="toggle-icon">
                                                <FolderIcon used={is_expanded} />
                                            </span>
                                            <h3 class="result-name">{ name }</h3>
                                            <span class="result-count">{ format!("{} 条匹配", match_count) }</span>
                                        </div>
                                        {
                                            if is_expanded {
                                                html! {
                                                     <ul class="result-lines">
                                                        { lines.iter().map(|(line_number, line_content)| {
                                                            html! {
                                                                <li class="result-line">
                                                                    <span class="line-number">{ format!("{} |", line_number) }</span>
                                                                    <span class="line-content ellipsis">{ line_content }</span>
                                                                </li>
                                                            }
                                                        }).collect::<Html>() }
                                                    </ul>
                                                }
                                            } else {
                                                html! {} // 收起状态不展示具体内容
                                            }
                                        }
                                    </li>
                                }
                            }).collect::<Html>() }
                        </ul>
                    }
                }
            }
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
            <div class="outline-tree">
                {file_tree.iter().map(|node| render_file_node(node, expanded_state.clone())).collect::<Html>()}
            </div>
        </div>
    }
}


#[function_component(FileView)]
fn file_view() -> Html {
    // 存储文件树数据
    let file_tree = use_state(|| None);
    // 存储展开状态
    let expanded_state = use_state(|| HashMap::<String, bool>::new());

    // 使用 use_effect 来加载数据，仅在组件挂载时触发一次，且只有当 file_tree 为空时加载
    {
        let file_tree = file_tree.clone();
        use_effect(move || {
            // 只有当 file_tree 为空时才触发请求
            if file_tree.is_none() {
                spawn_local(async move {
                    // 假设 load_tree_data 是你提供的异步加载数据函数
                    let tree = load_tree_data().await;
                    file_tree.set(Some(tree)); // 更新文件树数据
                });
            }
            // 返回一个清理函数（组件卸载时触发）
            || {}
        });
    }

    // 渲染文件树
    let render_tree = match &*file_tree {
        Some(tree) => tree.iter().map(|node| render_file_node(node, expanded_state.clone())).collect::<Html>(),
        None => html! { <div>{"Loading..."}</div> }, // 加载时显示 Loading
    };

    html! {
        <div class="sidebar-content">
            <div class="outline-tree">
                { render_tree }
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
                    if file_node.is_dir {
                        html! { <FolderIcon used={is_expanded} /> }
                    } else {
                        html! { <FileIcon /> }
                    }
                }
                <span class="file-node-name">{ &file_node.name }</span>
            </div>

            // 子节点（仅在文件夹展开时渲染）
            {
                if is_expanded && file_node.is_dir {
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
