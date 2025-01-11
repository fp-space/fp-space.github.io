
Welcome to Mini Note

采用 Rust + Yew 实现

## 目录结构

```
目录结构：
├── src/
│   ├── components/
│   │   ├── navigation.rs    # 导航栏组件
│   │   ├── sidebar.rs       # 目录树组件
│   │   ├── content.rs       # 内容展示区域
│   ├── context/
│   │   └── app_context.rs   # 应用的全局上下文（例如主题管理、用户状态、存储全局信息）
│   ├── main.rs              # 应用主入口
│   ├── generate.rs          # 生成 Markdown JSON文件
│   ├── app.css              # 样式文件，包含 Tailwind CSS
│   └── utils.rs             # 工具函数，可能包括文件加载和异步处理等
├── public/                  # 静态资源目录，存放图片、文件等
├── styles/                  # 存放 CSS、JS 文件的目录
├── Cargo.toml               # Rust 项目配置文件
└── README.md                # 项目说明文件
```
## 环境基础
### css
引入 twilwind, 使用 @apply，详见styles.scss

```bash
npx tailwindcss -i ./styles/app.scss -o ./styles/app.css --watch
```

### 文件树
```bash
cargo run --bin generate
```

## 需求 & 问题

1. github actions 部署 
2. 样式布局整理
3. latex 语法渲染问题，序号不对
4. 文件树排序问题
