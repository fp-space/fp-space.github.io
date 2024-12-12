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
        "folder-icon open text-yellow-500 text-xl"
    } else {
        "folder-icon text-gray-500 text-xl"
    };

    html! {
        <span class={class_name}>
            {
                if props.used {
                    html! {
                        <i class="fas fa-folder-open"></i>  // 用 Font Awesome 图标代替
                    }
                } else {
                    html! {
                        <i class="fas fa-folder"></i>  // 用 Font Awesome 图标代替
                    }
                }
            }
        </span>
    }
}



#[function_component(FileIcon)]
pub fn file_icon() -> Html {
    html! {
        <span class="file-icon text-gray-500 text-xl">
            <i class="fas fa-file"></i> // 使用 Font Awesome 的文件图标
        </span>
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