// 应用的全局上下文（例如主题管理、用户状态）

use std::rc::Rc;
use serde::{Deserialize, Serialize};
use yew::{function_component, html, use_reducer, ContextProvider, Html, Properties, Reducible, UseReducerHandle};
use crate::model::file_tree::FileNode;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub selected_file: Option<FileNode>,
}


impl Reducible for AppState {
    type Action = Option<FileNode>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        AppState { selected_file: action }.into()
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
        selected_file: None, // 初始化为 None
    });

    html! {
        <ContextProvider<AppStateContext> context={app_state}>
            {props.children.clone()}
        </ContextProvider<AppStateContext>>
    }
}