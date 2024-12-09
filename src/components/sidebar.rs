use yew::prelude::*;

// 文件树的数据结构
#[derive(Clone, PartialEq, Debug)]
pub struct FileTree {
    folder_name: &'static str,
    files: Vec<&'static str>,
    is_open: bool,
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    // 初始文件树数据
    let file_tree = use_state(|| vec![
        FileTree {
            folder_name: "src",
            files: vec!["main.rs", "lib.rs", "mod.rs"],
            is_open: false,
        },
        FileTree {
            folder_name: "assets",
            files: vec!["images", "styles"],
            is_open: false,
        },
        FileTree {
            folder_name: "components",
            files: vec!["header.rs", "footer.rs", "sidebar.rs"],
            is_open: false,
        },
    ]);

    // 切换文件夹的折叠状态
    let toggle_folder = {
        let file_tree = file_tree.clone(); // 克隆 file_tree 引用
        move |index: usize| {
            // 更新状态：修改文件树的折叠状态
            file_tree.set({
                let mut new_file_tree = (*file_tree).clone();
                let file = &mut new_file_tree[index];
                file.is_open = !file.is_open; // 切换折叠状态
                new_file_tree
            });
        }
    };

    html! {
        <div class="sidebar">
            <h3 class="text-xl font-semibold text-gray-800 mb-4">{"File Tree"}</h3>
            <ul class="file-tree">
                {
                    // 渲染文件树
                    for file_tree.iter().enumerate().map(|(index, file)| {
                        let toggle_folder = toggle_folder.clone(); // 在这里克隆一下 toggle_folder，防止它被移动
                        html! {
                            <li class="folder" key={index}>
                                <div class="folder-name" onclick={Callback::from(move |_| toggle_folder(index))}>
                                    {file.folder_name}
                                </div>
                                {
                                    // 如果文件夹是打开的，显示文件列表
                                    if file.is_open {
                                        html! {
                                            <ul class="file-list">
                                                { for file.files.iter().map(|file| html! { <li class="file">{file}</li> }) }
                                            </ul>
                                        }
                                    } else {
                                        html! { <ul class="file-list hidden"></ul> }
                                    }
                                }
                            </li>
                        }
                    })
                }
            </ul>
        </div>
    }
}
