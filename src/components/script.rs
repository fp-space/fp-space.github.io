use web_sys::window;

// 动态加载 MathJax
pub fn render_mathjax() {
    let window = window().unwrap();
    let document = window.document().unwrap();

    // 2. 创建 MathJax 配置脚本
    let script_config = document.create_element("script").unwrap();
    script_config.set_attribute("type", "text/x-mathjax-config").unwrap();
    script_config.set_inner_html(
        r#"
         var mathPackages = ['noundefined', 'autoload', 'ams', 'textmacros', 'xypic'];
    MathJax = {
            loader: {
                load: ['[custom]/xypic.js'],
                paths: {custom: 'https://cdn.jsdelivr.net/gh/sonoisa/XyJax-v3@3.0.1/build/'}
            },
            tex: {
                macros: {
                AA:"\u00c5",alef:"\\aleph",alefsym:"\\aleph",Alpha:"\\mathrm{A}",and:"\\land",ang:"\\angle",Bbb:"\\mathbb",Beta:"\\mathrm{B}",bold:"\\mathbf",bull:"\\bullet",C:"\\mathbb{C}",Chi:"\\mathrm{X}",clubs:"\\clubsuit",cnums:"\\mathbb{C}",Complex:"\\mathbb{C}",coppa:"\u03D9",Coppa:"\u03D8",Dagger:"\\ddagger",Digamma:"\u03DC",darr:"\\downarrow",dArr:"\\Downarrow",Darr:"\\Downarrow",diamonds:"\\diamondsuit",empty:"\\emptyset",Epsilon:"\\mathrm{E}",Eta:"\\mathrm{H}",euro:"\u20AC",exist:"\\exists",geneuro:"\u20AC",geneuronarrow:"\u20AC",geneurowide:"\u20AC",H:"\\mathbb{H}",hAar:"\\Leftrightarrow",harr:"\\leftrightarrow",Harr:"\\Leftrightarrow",hearts:"\\heartsuit",image:"\\Im",infin:"\\infty",Iota:"\\mathrm{I}",isin:"\\in",Kappa:"\\mathrm{K}",koppa:"\u03DF",Koppa:"\u03DE",lang:"\\langle",larr:"\\leftarrow",Larr:"\\Leftarrow",lArr:"\\Leftarrow",lrarr:"\\leftrightarrow",Lrarr:"\\Leftrightarrow",lrArr:"\\Leftrightarrow",Mu:"\\mathrm{M}",N:"\\mathbb{N}",natnums:"\\mathbb{N}",Nu:"\\mathrm{N}",O:"\\emptyset",officialeuro:"\u20AC",Omicron:"\\mathrm{O}",or:"\\lor",P:"\u00B6",pagecolor:["",1],part:"\\partial",plusmn:"\\pm",Q:"\\mathbb{Q}",R:"\\mathbb{R}",rang:"\\rangle",rarr:"\\rightarrow",Rarr:"\\Rightarrow",rArr:"\\Rightarrow",real:"\\Re",reals:"\\mathbb{R}",Reals:"\\mathbb{R}",Rho:"\\mathrm{P}",sdot:"\\cdot",sampi:"\u03E1",Sampi:"\u03E0",sect:"\\S",spades:"\\spadesuit",stigma:"\u03DB",Stigma:"\u03DA",sub:"\\subset",sube:"\\subseteq",supe:"\\supseteq",Tau:"\\mathrm{T}",textvisiblespace:"\u2423",thetasym:"\\vartheta",uarr:"\\uparrow",uArr:"\\Uparrow",Uarr:"\\Uparrow",varcoppa:"\u03D9",varstigma:"\u03DB",vline:"\\smash{\\large\\lvert}",weierp:"\\wp",Z:"\\mathbb{Z}",Zeta:"\\mathrm{Z}",
                dashint: "\\unicodeInt{x2A0D}",
                ddashint: "\\unicodeInt{x2A0E}",
                oiint: "\\unicodeInt{x222F}",
                oiiint: "\\unicodeInt{x2230}",
                ointctrclockwise: "\\unicodeInt{x2233}",
                unicodeInt: ["\\mathop{\\vcenter{\\mathchoice{\\huge\\unicode{#1}\\,}{\\unicode{#1}}{\\unicode{#1}}{\\unicode{#1}}}\\,}\\nolimits", 1],
                varointclockwise: "\\unicodeInt{x2232}",
            },
            maxBuffer: 10*1024,
            displayMath: [['$$\n', '\n$$'], ['$$\r\n', '\r\n$$']],
            inlineMath: [ ['$','$'], ['$$', '$$']],
            processEscapes: true,
            packages: {'[+]': mathPackages},
            tags: "all",
            formatError:               // function called when TeX syntax errors occur
                (jax, err) => {
                    var message = err.message.replace(/\n.*/, "");
                    return jax.parseOptions.nodeFactory.create("error", message, err.id, "Error: " + message);
                },
            useLabelIds: true
        },
        svg: {
            scale: 1,
            minScale: 80,
        },
        options: {
            skipHtmlTags: ["script","noscript","style","textarea","pre","code", "span"],
            processHtmlClass: "md-mathjax-preview|md-inline-math|inline-math-export-jax|math-in-toc",
            menuOptions: {
                settings: {
                    inTabOrder: false
                }
            },
            sre: {
                speech: 'shallow',      // or 'shallow', or 'deep'
            },
            enableMenu: false,
            ignoreHtmlClass: 'tex2jax_ignore',
            renderActions: {
                //
                // Force speech enrichment regardless of the menu settings
                // https://mathjax.github.io/MathJax-demos-web/speech-tex-chtml.html.html
                /*enrich: {'[+]': [
                    function (doc) {doc.enrich(true)},
                    function (math, doc) {math.enrich(doc, true)}
                ]}*/
            }
        }
    };
            "#,
    );

    // 插入 MathJax 脚本
    document.head().unwrap().append_child(&script_config).unwrap();



    // 启动 MathJax 渲染
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        &js_sys::Function::new_no_args("MathJax.typeset()"),
        1000
    ).unwrap();
}