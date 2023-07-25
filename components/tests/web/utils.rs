use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};

pub fn data_test_id(id: &str) -> String {
    let selector = format!("[data-testid=\"{}\"]", id);
    leptos::document()
        .query_selector(&selector)
        .unwrap()
        .expect("counters not found")
        .text_content()
        .unwrap()
}

pub fn find_by_text(text: &str) -> HtmlElement {
    let xpath = format!("//*[text()='{}']", text);
    let document = leptos::document();
    document
        .evaluate(&xpath, &document)
        .unwrap()
        .iterate_next()
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
}

pub trait HoverableElement {
    fn mouse_enter(&self);
    fn mouse_leave(&self);
}

impl HoverableElement for HtmlElement {
    fn mouse_enter(&self) {
        let mouse_event = MouseEvent::new("mouseenter").unwrap();
        self.dispatch_event(&mouse_event).unwrap();
    }

    fn mouse_leave(&self) {
        let mouse_event = MouseEvent::new("mouseleave").unwrap();
        self.dispatch_event(&mouse_event).unwrap();
    }
}
