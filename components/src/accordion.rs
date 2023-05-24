// use super::utils::*;
use leptos::*;
use uuid::Uuid;

#[derive(Clone, Debug)]
struct AccordionContext {
    // multiple: bool,
}

#[component]
pub fn Accordion(cx: Scope, children: Children, #[prop(optional)] class: String) -> impl IntoView {
    let state = AccordionContext {
        // multiple
    };

    let state_signal = create_rw_signal(cx, state);
    provide_context(cx, state_signal);

    view! { cx,
        <div role="presentation" class=class>
            {children(cx)}
        </div>
    }
}

#[derive(Clone, Debug, PartialEq)]
enum AccordionState {
    Open,
    Closed,
}

impl AccordionState {
    fn to_string(&self) -> &'static str {
        match self {
            AccordionState::Open => "true",
            AccordionState::Closed => "false",
        }
    }

    fn toggle(&self) -> Self {
        match self {
            AccordionState::Open => AccordionState::Closed,
            AccordionState::Closed => AccordionState::Open,
        }
    }
}

#[derive(Clone, Debug)]
struct AccordionItemContext {
    id: Uuid,
    state: AccordionState,
}

#[component]
pub fn AccordionItem(
    cx: Scope,
    children: Children,
    #[prop(optional)] class: String,
) -> impl IntoView {
    let id = Uuid::new_v4();

    let state = AccordionItemContext {
        id,
        state: AccordionState::Closed,
    };

    let state_signal = create_rw_signal(cx, state);
    provide_context(cx, state_signal);

    view! { cx,
        <div class=class>
            {children(cx)}
        </div>
    }
}

#[component]
pub fn AccordionTrigger(
    cx: Scope,
    children: Children,
    #[prop(optional)] class: String,
) -> impl IntoView {
    let accordion_state = use_context::<RwSignal<AccordionContext>>(cx)
        .expect("to be used in an Accordion")
        .clone();

    let state =
        use_context::<RwSignal<AccordionItemContext>>(cx).expect("to be used in an AccordionItem");

    let id = move || state.get().id;
    let open = move || state.get().state == AccordionState::Open;

    let handle_click = move |_| {
        state.update(|s| {
            log!("{:?}", s);
            s.state = s.state.toggle();
        })
    };

    let id_string = move || id().to_string();
    view! { cx,
        <button
            id=format!("{}-trigger", id_string())
            aria-controls=format!("{}-content", id_string())
            aria-expanded=move || open().to_string()
            tabindex="0"
            on:click=handle_click
            class=class
        >
            {children(cx)}
        </button>
    }
}

#[component]
pub fn AccordionContent(
    cx: Scope,
    children: ChildrenFn,
    #[prop(optional)] class: String,
) -> impl IntoView {
    let accordion_state = use_context::<RwSignal<AccordionContext>>(cx)
        .expect("to be used in an Accordion")
        .clone();

    let state =
        use_context::<RwSignal<AccordionItemContext>>(cx).expect("to be used in an AccordionItem");

    let id = move || state.get().id;
    let hidden = move || state.get().state == AccordionState::Closed;

    let id_string = move || id().to_string();

    let children = store_value(cx, children);
    view! { cx,
        <div
            id=format!("{}-content", id_string())
            aria-labelledby=format!("{}-trigger", id_string())
            aria-hidden=move || hidden().to_string()
            tabindex="-1"
            role="region"
            class=class
        >
            <Show when=move || !hidden() fallback=|_| ()>
                {children.with_value(|children| children(cx))}
            </Show>
        </div>
    }
}
