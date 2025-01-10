// 应用的全局上下文（例如主题管理、用户状态）

use crate::model::file_tree::FileNode;
use crate::model::outline_tree::TitleNode;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::{function_component, html, use_reducer, ContextProvider, Html, Properties, Reducible, UseReducerHandle};
use crate::model::tree::TreeNode;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub selected_file: Option<FileNode>,
    #[serde(skip)]  // 跳过该字段的反序列化
    pub outline: Option<Vec<TreeNode<TitleNode>>>,
}


// 定义更新应用状态的 Action 类型，支持不同的更新操作
#[derive(Debug, Clone)]
pub enum AppStateAction {
    UpdateSelectedFile(Option<FileNode>),
    UpdateUserStatus(Option<Vec<TreeNode<TitleNode>>>),
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppStateAction::UpdateSelectedFile(file) => {
                AppState { selected_file: file, outline: self.outline.clone() }.into()
            }
            AppStateAction::UpdateUserStatus(outline) => {
                AppState { selected_file: self.selected_file.clone(), outline }.into()
            }
        }
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;

#[derive(Properties, Debug, PartialEq)]
pub struct MessageProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn AppStateProvider(props: &MessageProviderProps) -> Html {
    let app_state = use_reducer(|| AppState {
        selected_file: None,
        outline: None,
    });
    html! {
        <ContextProvider<AppStateContext> context={app_state}>
            {props.children.clone()}
        </ContextProvider<AppStateContext>>
    }
}