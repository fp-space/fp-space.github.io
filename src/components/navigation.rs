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
       <nav class="navigation">
            <div class="navigation-container">
                // 左侧 Logo/图标部分
                <a class="navigation-logo" href="/">
                    <svg class="navigation-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                    </svg>
                    <span class="navigation-title">{"Mini Note"}</span>
                </a>

                // 右侧导航链接
                <div class="navigation-links">
                    {NAVMENU.iter().map(|item| {
                        html! {
                            <a href={item.url} class="navigation-link">
                                {&item.name}
                            </a>
                        }
                    }).collect::<Html>()}
                </div>
            </div>
        </nav>
    }
}