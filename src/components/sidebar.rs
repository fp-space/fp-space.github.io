use std::cell::RefCell;
use crate::components::icon::{FileIcon, FolderIcon};
use crate::context::app_context::{AppStateAction, AppStateContext};
use crate::model::file_tree::FileNode;
use crate::model::outline_tree::{sanitize_title, TitleNode};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::model::tree::TreeNode;

#[derive(PartialEq, Clone)]
enum Tab {
    File,
    Outline
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let active_tab = use_state(|| Tab::File);

    let switch_tab = |tab: Tab| {
        let active_tab = active_tab.clone();
        Callback::from(move |_: MouseEvent| {
            active_tab.set(tab.clone());
        })
    };

    html! {
        <div class="sidebar">
            <div class="sidebar-tabs">
                <TabButton 
                    active={*active_tab == Tab::File} 
                    label="文件" 
                    onclick={switch_tab(Tab::File)} 
                />
                <TabButton 
                    active={*active_tab == Tab::Outline} 
                    label="大纲" 
                    onclick={switch_tab(Tab::Outline)} 
                />
            </div>

            <div class="sidebar-view">
                <TabContent active={*active_tab == Tab::File}>
                    <FileView />
                </TabContent>
                <TabContent active={*active_tab == Tab::Outline}>
                    <OutlineView />
                </TabContent>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TabButtonProps {
    active: bool,
    label: String,
    onclick: Callback<MouseEvent>,
}

#[function_component(TabButton)]
fn tab_button(props: &TabButtonProps) -> Html {
    let class = if props.active {
        "sidebar-tab active"
    } else {
        "sidebar-tab"
    };

    html! {
        <button class={class} onclick={props.onclick.clone()}>
            { &props.label }
        </button>
    }
}

#[derive(Properties, PartialEq)]
struct TabContentProps {
    active: bool,
    children: Children,
}

#[function_component(TabContent)]
fn tab_content(props: &TabContentProps) -> Html {
    let class = if props.active { "tab-content active" } else { "tab-content" };

    html! {
        <div class={class}>
            { props.children.clone() }
        </div>
    }
}

#[function_component(OutlineView)]
fn outline_view() -> Html {
    let app_state_ctx = use_context::<AppStateContext>().unwrap();
    let expanded_state = use_state(|| HashMap::<String, bool>::new());
    let outline_data = app_state_ctx.outline.clone().unwrap_or_default();

    let render_outline = outline_data.iter().map(|root| render_outline_node(root.clone(), &expanded_state)).collect::<Html>();

    html! {
        <div class="sidebar-content">
            <div class="outline-tree">
                { render_outline }
            </div>
        </div>
    }
}

fn render_outline_node(
    node: Rc<RefCell<TitleNode>>,
    expanded_state: &UseStateHandle<HashMap<String, bool>>
) -> Html {
    let node_ref = node.borrow_mut();
    let title = node_ref.title.clone();
    let is_expanded = expanded_state.get(&title).cloned().unwrap_or(false);

    let toggle = {
        let expanded_state = expanded_state.clone();
        let title = title.clone();
        Callback::from(move |_| {
            let mut state = (*expanded_state).clone();
            let is_expanded = state.entry(title.clone()).or_insert(false);
            *is_expanded = !*is_expanded;
            expanded_state.set(state);
        })
    };

    html! {
        <div class="outline-node">
            <div class="outline-node-header" onclick={toggle}>
                <span class="outline-node-name">
                    <a href={format!("#{}", sanitize_title(&title))}>{title}</a>
                </span>
                { render_toggle_button(is_expanded, node_ref.has_children) }
            </div>
            { render_children(is_expanded, &node_ref.children, expanded_state) }
        </div>
    }
}

fn render_toggle_button(is_expanded: bool, has_children: bool) -> Html {
    if has_children {
        html! {
            <span class="outline-node-toggle">
                <i class={if is_expanded { "fas fa-chevron-down" } else { "fas fa-chevron-right" }}></i>
            </span>
        }
    } else {
        html! {}
    }
}

fn render_children(
    is_expanded: bool,
    children: &Vec<Rc<RefCell<TitleNode>>>,
    expanded_state: &UseStateHandle<HashMap<String, bool>>
) -> Html {
    if is_expanded {
        html! {
            <div class="outline-node-children">
                {
                    children.iter()
                        .map(|child| render_outline_node(child.clone(), expanded_state))
                        .collect::<Html>()
                }
            </div>
        }
    } else {
        html! {}
    }
}

#[function_component(FileView)]
fn file_view() -> Html {
    let file_tree = use_state(|| None);
    let expanded_state = use_state(|| HashMap::<String, bool>::new());
    let app_state_ctx = use_context::<AppStateContext>().unwrap();

    {
        let file_tree = file_tree.clone();
        use_effect(move || {
            if file_tree.is_none() {
                spawn_local(async move {
                    let tree = FileNode::load_tree_data().await;
                    file_tree.set(Some(tree));
                });
            }
            || {}
        });
    }

    let render_tree = match &*file_tree {
        Some(tree) => tree.iter().map(|node| render_file_node(node, &app_state_ctx, expanded_state.clone())).collect::<Html>(),
        None => html! { <div class="loading">{"Loading..."}</div> },
    };

    html! {
        <div class="sidebar-content">
            <div class="file-tree">
                { render_tree }
            </div>
        </div>
    }
}

fn render_file_node(tree_node: &TreeNode<FileNode>, app_state_ctx: &AppStateContext, expanded_state: UseStateHandle<HashMap<String, bool>>) -> Html {
    let file_node = tree_node.clone().data;
    let file_name = file_node.name.clone();
    let is_expanded = expanded_state.get(&file_name).cloned().unwrap_or(false);

    let toggle = {
        let expanded_state = expanded_state.clone();
        let file_name = file_name.clone();
        let file_node = file_node.clone();
        let app_state_ctx = app_state_ctx.clone();

        Callback::from(move |_| {
            let mut state = (*expanded_state).clone();
            let is_expanded = state.entry(file_name.clone()).or_insert(false);
            *is_expanded = !*is_expanded;
            expanded_state.set(state);

            if !file_node.is_dir {
                app_state_ctx.dispatch(AppStateAction::UpdateSelectedFile(Some(file_node.clone())));
            }
        })
    };

    html! {
        <div class="file-node">
            <div class="file-node-header" onclick={toggle}>
                {
                    if file_node.is_dir {
                        html! {
                            <div class="file-node-folder">
                                <FolderIcon used={is_expanded} />
                                <span class="file-node-name">{ &file_node.name }</span>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="file-node-file">
                                <FileIcon />
                                <span class="file-node-name"><a href="#mini-note-top">{ &file_node.name }</a></span>
                            </div>
                        }
                    }
                }
            </div>

            {
                if is_expanded && file_node.is_dir {
                    html! {
                        <div class="file-node-children">
                            {
                                tree_node.children.as_ref()
                                    .unwrap_or(&vec![])
                                    .iter()
                                    .map(|child| render_file_node(child, app_state_ctx, expanded_state.clone()))
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