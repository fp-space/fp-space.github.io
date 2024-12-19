mod markdown;
mod utils;
mod components;
mod model;
mod context;

use crate::components::content::MainContent;
use crate::components::navigation::Navigation;
use crate::components::sidebar::Sidebar;
use console_log::init_with_level;
use log::Level;
use yew::{function_component, html, use_effect, Html, Renderer};
use crate::context::app_context::AppStateProvider;


fn render_mathjax() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // 1. 创建 MathJax 脚本标签
    let script_mathjax = document.create_element("script").unwrap();
    script_mathjax.set_attribute("type", "text/javascript").unwrap();
    script_mathjax.set_attribute("id", "MathJax-script").unwrap();
    script_mathjax.set_attribute("async", "true").unwrap();
    script_mathjax.set_attribute("src", "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js").unwrap();

    // 将 MathJax 脚本插入到 <head> 中
    let head = document.head().unwrap();
    head.append_child(&script_mathjax).unwrap();

    // 2. 创建 MathJax 配置脚本
    let script_config = document.create_element("script").unwrap();
    script_config.set_attribute("type", "text/x-mathjax-config").unwrap();
    script_config.set_inner_html(
        r#"
    var mathPackages = ['noundefined', 'autoload', 'ams', 'textmacros', 'xypic'];
    if((window._options || {}).allowPhysicsConflict) {
        mathPackages.push('physics');
    }

    var autoNumberingForMath = (window._options || {}).autoNumberingForMath;
    var mathJaxTags = autoNumberingForMath ? "all" : "none";
    if(autoNumberingForMath == "ams") mathJaxTags = "ams";
    if(autoNumberingForMath == "0" || autoNumberingForMath == "null" || autoNumberingForMath == "false" || autoNumberingForMath == "nil") mathJaxTags = "none";


    window.MathJax = {
        loader: {load: [/*'ui/safe', 'a11y/semantic-enrich',*/ 'http://sonoisa.github.io/xyjax_ext/xypic.js']},
        startup: {
            typeset: false,
            ready: () => {
                console.debug('MathJax is loaded, but not yet initialized');

                const {RANGES} = MathJax._.core.MmlTree.OperatorDictionary;
                RANGES[28][3] = RANGES[30][3] = RANGES[33][3] = RANGES[47][3] = 'mtext';
                MathJax.startup.defaultReady();
                MathJax.startup.document.addStyleSheet();
            },
            pageReady: () => {
                /*var safeOptions = MathJax.startup.document.safe.options;
                safeOptions.safeProtocols.file = false;
                safeOptions.idPattern = /^mjx-[-a-zA-Z0-9_.:]+$/;
                safeOptions.dataPattern = /^data-(mjx|semantic)-/;*/
                //MathJax.startup.document.options.internalSpeechTitles = true;
                console.debug('MathJax is initialized');
            }
        },
        tex: {
            macros: {
                /*
                    MathJax.startup.input[0].configuration.handlers.get("macro")._configuration.items
                */
                // https://github.com/mathjax/MathJax/pull/1810/files
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
            tags: mathJaxTags,
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

#[function_component(App)]
fn app() -> Html {

    // 在组件挂载时动态插入 MathJax 脚本及配置
    use_effect(|| {
        render_mathjax();
    });


    html! {
        <AppStateProvider>
            <div class="app-container">
                <div class="app-container-sidebar">
                    <Sidebar />
                </div>

                <div class="main-content">
                    <div class="main-content-navbar">
                        <Navigation />
                    </div>

                    <div class="main-content-area">
                        <MainContent />
                    </div>
                </div>
            </div>
        </AppStateProvider>
    }
}


fn main() {
    // Initialize logging for WASM
    init_with_level(Level::Info).expect("failed to initialize logger");

    // Render the Yew app
    Renderer::<App>::new().render();
}