fn main() {
    bitop_ui_build::Build::new("src/components")
        .global("src/styles/tokens.css")
        .global("src/styles/theme-neutral.css")
        .global("src/styles/icons.css")
        .global("src/styles/global.css")
        .stylesheet("bitop-ui.css")
        .script_prelude("src/script-prelude.js")
        .script("bitop-ui.js")
        .run()
        .unwrap_or_else(|error| panic!("{error}"));
}
