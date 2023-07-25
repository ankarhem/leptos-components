use leptos::*;
use leptos_a11y::listbox::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, NodeList};

#[component]
fn ListboxExample<T>(cx: Scope, options: Vec<T>) -> impl IntoView
where
    T: IntoView + Clone + Copy + 'static + PartialEq,
{
    // let options = vec!["one", "two", "three", "four"];
    let value = create_rw_signal(cx, options[0]);

    view! { cx,
        <section id="test-wrapper">
        <Listbox value=value>
            <ListboxButton>
                {value}
            </ListboxButton>
            <ListboxOptions clone:options>
                {options.clone().into_iter().map(|value| {
                    view! { cx,
                        <ListboxOption value=value>
                            {value}
                        </ListboxOption>
                    }
                }).collect::<Vec<_>>()}
            </ListboxOptions>
        </Listbox>
        </section>
    }
}

fn remove_existing_test() {
    let selector = "#test-wrapper".to_string();
    let test_wrapper = leptos::document().query_selector(&selector).unwrap();
    if let Some(test_wrapper) = test_wrapper {
        test_wrapper.remove();
    }
}

pub fn render_listbox<T>(options: Vec<T>)
where
    T: IntoView + Clone + Copy + 'static + PartialEq,
{
    remove_existing_test();
    mount_to_body(move |cx| view! { cx, <ListboxExample options=options /> })
}

pub fn listbox_button_element() -> Element {
    let selector = "button[aria-haspopup='true']".to_string();
    leptos::document()
        .query_selector(&selector)
        .unwrap()
        .unwrap()
}

pub fn listbox_button_html_element() -> HtmlElement {
    listbox_button_element().dyn_into::<HtmlElement>().unwrap()
}

pub fn listbox_options_element() -> Element {
    let selector = "ul[role='listbox']".to_string();
    leptos::document()
        .query_selector(&selector)
        .unwrap()
        .unwrap()
}

pub fn listbox_options_html_element() -> HtmlElement {
    listbox_options_element().dyn_into::<HtmlElement>().unwrap()
}

pub fn listbox_option_node_list() -> NodeList {
    let selector = "li[role='option']".to_string();
    leptos::document().query_selector_all(&selector).unwrap()
}
