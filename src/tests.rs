use super::*;

#[cfg(feature = "wasm-js")]
use wasm_bindgen_test::wasm_bindgen_test as test;

#[cfg(all(feature = "wasm-js", feature = "wasm-js-test-in-browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn test_render() {
    let html = render("a = b + c").unwrap();
    assert!(!html.contains(r#"span class="katex-display""#));
    assert!(html.contains(r#"span class="katex""#));
    assert!(html.contains(r#"span class="katex-mathml""#));
    assert!(html.contains(r#"span class="katex-html""#));
    assert!(!html.contains(r#"span class="katex-error""#));
}

#[test]
fn test_render_mhchem() {
    let html = render(r#"\ce{CO2 + C -> 2 CO}"#).unwrap();
    assert!(!html.contains(r#"span class="katex-display""#));
    assert!(html.contains(r#"span class="katex""#));
    assert!(html.contains(r#"span class="katex-mathml""#));
    assert!(html.contains(r#"span class="katex-html""#));
    assert!(!html.contains(r#"span class="katex-error""#));
}

#[test]
fn test_passing_opts_by_reference_and_value() {
    let opts = Opts::builder().display_mode(true).build().unwrap();
    let html1 = render_with_opts("a = b + c", &opts).unwrap();
    let html2 = render_with_opts("a = b + c", opts).unwrap();
    assert_eq!(html1, html2);
}

#[test]
fn test_display_mode() {
    let opts = Opts::builder().display_mode(true).build().unwrap();
    let html = render_with_opts("a = b + c", &opts).unwrap();
    assert!(html.contains(r#"span class="katex-display""#));
}

#[test]
fn test_output_html_only() {
    let opts = Opts::builder()
        .output_type(OutputType::Html)
        .build()
        .unwrap();
    let html = render_with_opts("a = b + c", &opts).unwrap();
    assert!(!html.contains(r#"span class="katex-mathml""#));
    assert!(html.contains(r#"span class="katex-html""#));
}

#[test]
fn test_output_mathml_only() {
    let opts = Opts::builder()
        .output_type(OutputType::Mathml)
        .build()
        .unwrap();
    let html = render_with_opts("a = b + c", &opts).unwrap();
    assert!(html.contains(r#"MathML"#));
    assert!(!html.contains(r#"span class="katex-html""#));
}

#[test]
fn test_leqno() {
    let opts = Opts::builder()
        .display_mode(true)
        .leqno(true)
        .build()
        .unwrap();
    let html = render_with_opts("a = b + c", &opts).unwrap();
    assert!(html.contains(r#"span class="katex-display leqno""#));
}

#[test]
fn test_fleqn() {
    let opts = Opts::builder()
        .display_mode(true)
        .fleqn(true)
        .build()
        .unwrap();
    let html = render_with_opts("a = b + c", &opts).unwrap();
    assert!(html.contains(r#"span class="katex-display fleqn""#));
}

#[test]
fn test_throw_on_error() {
    let err_msg = match render(r#"\"#) {
        Ok(_) => unreachable!(),
        Err(e) => match e {
            Error::JsExecError(msg) => msg,
            _ => unreachable!(),
        },
    };
    assert!(err_msg.contains("ParseError"));
}

#[test]
fn test_error_color() {
    let opts = Opts::builder()
        .throw_on_error(false)
        .error_color("#ff0000")
        .build()
        .unwrap();
    let html = render_with_opts(r#"\"#, &opts).unwrap();
    assert!(html.contains(r#"span class="katex-error""#));
    assert!(html.contains("color:#ff0000"));
}

#[test]
fn test_macros() {
    let opts = Opts::builder()
        .add_macro(r#"\RR"#.to_owned(), r#"\mathbb{R}"#.to_owned())
        .build()
        .unwrap();
    let html = render_with_opts(r#"\RR"#, &opts).unwrap();
    assert!(html.contains("mathbb"));
}

#[test]
fn test_trust() {
    let opts = Opts::builder().error_color("#ff0000").build().unwrap();
    let html = render_with_opts(r#"\url{https://www.google.com}"#, &opts).unwrap();
    assert!(html.contains(r#"color:#ff0000"#));
    assert!(!html.contains(r#"a href="https://www.google.com""#));

    let opts = Opts::builder()
        .error_color("#ff0000")
        .trust(true)
        .build()
        .unwrap();
    let html = render_with_opts(r#"\url{https://www.google.com}"#, &opts).unwrap();
    assert!(!html.contains(r#"color:#ff0000"#));
    assert!(html.contains(r#"a href="https://www.google.com""#));
}

#[test]
fn test_stack_overflow() {
    #[inline(never)]
    fn simulate_deep_stack(i: i32) {
        if i > 0 {
            simulate_deep_stack(i - 1);
        } else {
            let html = render("a = b + c").unwrap();
            assert!(html.contains(r#"span class="katex""#));
        }
    }
    simulate_deep_stack(100);
    simulate_deep_stack(0);
}

#[test]
fn test_opts_sync_send() {
    fn is_sync_send<T: Sync + Send>(_: T) {}
    let opts = Opts::builder().build().unwrap();
    is_sync_send(opts);
}
