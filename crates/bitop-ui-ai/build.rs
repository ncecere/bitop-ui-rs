fn main() {
    // No global styles: tokens, the theme and the base styles come from bitop-ui, whose
    // stylesheet must load first. bitop_ui_ai::head() renders after bitop_ui::head().
    bitop_ui_build::Build::new("src/components")
        .stylesheet("bitop-ui-ai.css")
        .script_prelude("src/script-prelude.js")
        .script("bitop-ui-ai.js")
        .run()
        .unwrap_or_else(|error| panic!("{error}"));
}
