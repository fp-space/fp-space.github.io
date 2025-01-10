use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct FolderIconProps {
    pub used: bool,
}

// 文件夹图标
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

// 文件图标
#[function_component(FileIcon)]
pub fn file_icon() -> Html {
    html! {
        <span class="file-icon text-gray-500 text-xl">
            <i class="fas fa-file"></i> // 使用 Font Awesome 的文件图标
        </span>
    }
}