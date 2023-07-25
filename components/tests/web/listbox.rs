use super::*;
use crate::fixtures::listbox as ui;
use pretty_assertions::assert_eq;

#[wasm_bindgen_test]
fn should_not_render_options_before_opened() {
    ui::render_listbox();

    let options = ui::listbox_option_node_list();

    assert_eq!(options.length(), 0);
}

#[wasm_bindgen_test]
fn should_render_options_after_opened() {
    ui::render_listbox();
    ui::listbox_button_html_element().click();

    let options = ui::listbox_option_node_list();

    assert_eq!(options.length(), 4);
}
