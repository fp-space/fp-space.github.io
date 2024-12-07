Mini Note

采用 Rust 编程语言，选择 Yew 框架

## 目录结构

```
目录结构：
my_blog_project/
├── src/
│   ├── components/
│   │   ├── header.rs        # 导航栏组件
│   │   ├── sidebar.rs       # 目录树组件
│   │   ├── content.rs       # 内容展示区域
│   ├── routes/
│   │   ├── home.rs          # 首页路由
│   │   ├── post.rs          # 单个笔记页面路由
│   ├── context/
│   │   └── app_context.rs   # 应用的全局上下文（例如主题管理、用户状态）
│   ├── main.rs              # 应用主入口
│   ├── markdown.rs          # 处理 Markdown 文件的加载与渲染
│   ├── app.css              # 样式文件，包含 Tailwind CSS
│   └── utils.rs             # 工具函数，可能包括文件加载和异步处理等
├── static/                  # 静态资源目录，存放图片、文件等
│   ├── assets/              # 其他静态资源，如图标、图片等
│   └── posts/               # 存放 Markdown 文件的目录
├── Cargo.toml               # Rust 项目配置文件
└── README.md                # 项目说明文件
```
