use super::*;
use crate::fixtures::listbox as ui;
use crate::utils::*;
use pretty_assertions::assert_eq;

const OPTIONS: [&str; 4] = ["one", "two", "three", "four"];

// FUNCTIONALITY TESTS - BEGIN

#[wasm_bindgen_test]
fn should_not_render_options_before_opened() {
    ui::render_listbox(OPTIONS);

    let options = ui::listbox_option_node_list();

    assert_eq!(options.length(), 0);
}

#[wasm_bindgen_test]
fn should_render_options_after_opened() {
    ui::render_listbox(OPTIONS);
    ui::listbox_button_html_element().click();

    let options = ui::listbox_option_node_list();

    assert_eq!(options.length(), 4);
}

#[wasm_bindgen_test]
fn first_option_is_selected_by_default() {
    ui::render_listbox(OPTIONS);
    let button_text = ui::listbox_button_html_element().inner_text();

    assert_eq!(button_text, OPTIONS[0].to_string());
}

#[wasm_bindgen_test]
fn can_select_an_option() {
    ui::render_listbox(OPTIONS);

    ui::listbox_button_html_element().click();
    find_by_text(OPTIONS[1]).click();

    let button_text = ui::listbox_button_html_element().inner_text();

    assert_eq!(button_text, OPTIONS[1].to_string());
}

#[wasm_bindgen_test]
fn should_set_data_active_on_selected_element_by_default() {
    ui::render_listbox(OPTIONS);

    ui::listbox_button_html_element().click();
    let selected_option = ui::listbox_selected_option_html_element();

    assert_eq!(
        selected_option.get_attribute("data-active"),
        Some("true".into())
    );
    assert_eq!(
        leptos::document()
            .query_selector_all("[data-active='true']")
            .unwrap()
            .length(),
        1
    );
}

#[wasm_bindgen_test]
fn should_toggle_data_active_on_hover() {
    ui::render_listbox(OPTIONS);

    ui::listbox_button_html_element().click();
    let selected_option = ui::listbox_selected_option_html_element();

    assert_eq!(
        selected_option.get_attribute("data-active"),
        Some("true".into())
    );

    let el_two = find_by_text(OPTIONS[1]);
    el_two.mouse_enter();
    assert_eq!(el_two.get_attribute("data-active"), Some("true".into()));
    el_two.mouse_leave();
    assert_eq!(
        leptos::document()
            .query_selector_all("[data-active='true']")
            .unwrap()
            .length(),
        0
    );
}

// FUNCTIONALITY TESTS - END

// ARIA TESTS - BEGIN

#[wasm_bindgen_test]
fn should_set_aria_haspopup() {
    ui::render_listbox(OPTIONS);
    let button = ui::listbox_button_html_element();

    assert_eq!(button.get_attribute("aria-haspopup"), Some("true".into()));
}

#[wasm_bindgen_test]
fn should_toggle_aria_expanded() {
    ui::render_listbox(OPTIONS);
    let button = ui::listbox_button_html_element();

    assert_eq!(button.get_attribute("aria-expanded"), Some("false".into()));
    button.click();
    assert_eq!(button.get_attribute("aria-expanded"), Some("true".into()));
}

#[wasm_bindgen_test]
fn should_have_matching_id_for_aria_controls() {
    ui::render_listbox(OPTIONS);
    let button = ui::listbox_button_html_element();
    button.click();
    let ul = ui::listbox_options_html_element();

    assert_eq!(button.get_attribute("aria-controls"), Some(ul.id()));
}

#[wasm_bindgen_test]
fn should_set_aria_selected() {
    ui::render_listbox(OPTIONS);
    let button = ui::listbox_button_html_element();
    button.click();
    let selected_option = ui::listbox_selected_option_html_element();

    assert_eq!(
        selected_option.get_attribute("aria-selected"),
        Some("true".into())
    );
    assert_eq!(
        leptos::document()
            .query_selector_all("[aria-selected='true']")
            .unwrap()
            .length(),
        1
    );
}

// ARIA TESTS - END
