use yew::{function_component, html, Html, Properties};

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


#[function_component(SearchIcon)]
pub fn search_icon() -> Html {
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