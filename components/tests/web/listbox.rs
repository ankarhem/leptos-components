use super::*;
use crate::fixtures::listbox as ui;
use pretty_assertions::assert_eq;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

// Functional tests

#[wasm_bindgen_test]
fn should_not_render_options_before_opened() {
    let options = vec!["one", "two", "three", "four"];
    ui::render_listbox(options);

    let options = ui::listbox_option_node_list();

    assert_eq!(options.length(), 0);
}

#[wasm_bindgen_test]
fn should_render_options_after_opened() {
    let options = vec!["one", "two", "three", "four"];
    ui::render_listbox(options);
    ui::listbox_button_html_element().click();

    let options = ui::listbox_option_node_list();

    assert_eq!(options.length(), 4);
}

#[wasm_bindgen_test]
fn first_option_is_selected_by_default() {
    let options = vec!["one", "two", "three", "four"];
    ui::render_listbox(options.clone());
    let button_text = ui::listbox_button_html_element().inner_text();

    assert_eq!(button_text, options[0].to_string());
}

#[wasm_bindgen_test]
fn can_select_an_option() {
    let options = vec!["one", "two", "three", "four"];
    ui::render_listbox(options.clone());

    ui::listbox_button_html_element().click();
    ui::listbox_option_node_list()
        .item(1)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    let button_text = ui::listbox_button_html_element().inner_text();

    assert_eq!(button_text, options[1].to_string());
}

// Aria tests
#[wasm_bindgen_test]
fn should_have_aria_haspopup() {
    let options = vec!["one", "two", "three", "four"];
    ui::render_listbox(options);
    let button = ui::listbox_button_html_element();

    assert_eq!(button.get_attribute("aria-haspopup"), Some("true".into()));
}

#[wasm_bindgen_test]
fn should_toggle_aria_expanded() {
    let options = vec!["one", "two", "three", "four"];
    ui::render_listbox(options);
    let button = ui::listbox_button_html_element();

    assert_eq!(button.get_attribute("aria-expanded"), Some("false".into()));
    button.click();
    assert_eq!(button.get_attribute("aria-expanded"), Some("true".into()));
}

#[wasm_bindgen_test]
fn should_have_matching_id_for_aria_controls() {
    let options = vec!["one", "two", "three", "four"];
    ui::render_listbox(options);
    let button = ui::listbox_button_html_element();
    button.click();
    let ul = ui::listbox_options_html_element();

    assert_eq!(button.get_attribute("aria-controls"), Some(ul.id()));
}
