use web_sys::window;

pub fn render_mathjax() {
    let window = window().unwrap();

    // 启动 MathJax 渲染并重置序号
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        &js_sys::Function::new_no_args(r#"
            MathJax.typesetPromise().then(() => {
                console.log('MathJax rendering complete');
            }).catch((err) => {
                console.error('MathJax rendering error:', err);
            });
        "#),
        1000
    ).unwrap();
}

