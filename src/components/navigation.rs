// navigation.rs
use yew::prelude::*;

#[derive(Clone)]
pub struct NavMenu {
    pub name: &'static str,
    pub url: &'static str,
}

pub const NAVMENU: &[NavMenu] = &[
    NavMenu {
        name: "ARCHIVE",
        url: "/archives/",
    },
    NavMenu {
        name: "TAG",
        url: "/tag/",
    },
    NavMenu {
        name: "ABOUT",
        url: "/about",
    },
];

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
       <nav class="bg-stone-100 text-gray-800 py-5 shadow-md">
            <div class="flex items-center justify-between px-20">
                // 左侧 Logo/图标部分
                <a class="flex items-center" href="/">
                    <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                    </svg>
                    <span class="ml-2 text-lg font-semibold">{"Mini Note"}</span>
                </a>

                // 右侧导航链接
                <div class="flex items-center space-x-6">
                    {NAVMENU.iter().map(|item| {
                        html! {
                            <a href={item.url} class="hover:text-blue-600 font-medium transition-colors">
                                {&item.name}
                            </a>
                        }
                    }).collect::<Html>()}
                </div>
            </div>
        </nav>
    }
}
