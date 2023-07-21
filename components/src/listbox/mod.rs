use leptos::html::Li;
use leptos::leptos_dom::console_log;
use leptos::*;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct ListboxContext {
    id: Uuid,
    open: RwSignal<bool>,
    selected: RwSignal<Option<Uuid>>,
    active: RwSignal<Option<Uuid>>,
}

impl ListboxContext {
    fn new(cx: Scope) -> Self {
        Self {
            id: Uuid::new_v4(),
            open: create_rw_signal(cx, false),
            selected: create_rw_signal(cx, None),
            active: create_rw_signal(cx, None),
        }
    }
}

#[component]
pub fn Listbox(cx: Scope, children: ChildrenFn, #[prop(optional)] class: String) -> impl IntoView {
    let context = ListboxContext::new(cx);
    provide_context(cx, context);

    view! { cx,
        <div class=class>
            {children(cx)}
        </div>
    }
}

#[component]
pub fn ListboxButton(
    cx: Scope,
    children: Children,
    #[prop(optional)] class: String,
) -> impl IntoView {
    let context = use_context::<ListboxContext>(cx).unwrap();

    let open = move || context.open.get();
    let on_click = move |_| {
        context.open.set(!open());
    };

    view! { cx,
        <button
            class=class
            on:click=on_click
            aria-haspopup="true"
            aria-expanded=move || open().to_string()
            aria-controls=context.id.to_string()
        >
            {children(cx)}
        </button>
    }
}

#[component]
pub fn ListboxOptions(cx: Scope, children: ChildrenFn) -> impl IntoView {
    let context = use_context::<ListboxContext>(cx).unwrap();

    let active = move || context.active.get().map(|id| id.to_string());
    let open = move || context.open.get();

    view! { cx,
        <Show
            when=open
            fallback=|_cx| view! { cx, <></> }
        >
            <ul
                id=context.id.to_string()
                role="listbox"
                tabindex="0"
                aria-orientation="vertical"
                aria-activedescendant=active
            >
                {children(cx)}
            </ul>
        </Show>
    }
}

#[component]
pub fn ListboxOption(
    cx: Scope,
    children: ChildrenFn,
    #[prop(optional)] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let id = Uuid::new_v4();
    let el = create_node_ref::<Li>(cx);
    let context = use_context::<ListboxContext>(cx).unwrap();

    let selected = move || context.selected.get() == Some(id);

    let on_click = move |_| {
        if !selected() {
            context.selected.set(Some(id));
            // context.open.set(false);
        }
    };

    let active = move || context.active.get() == Some(id);
    let on_mouse_enter = move |_| {
        if !active() {
            context.active.set(Some(id));
        }
    };
    let on_mouse_leave = move |_| {
        if active() {
            context.active.set(None);
        }
    };

    view! { cx,
        <li
            id=id.to_string()
            node_ref=el
            role="option"
            tabindex="-1"
            class=class
            on:click=on_click
            disabled=move || disabled
            aria-selected=move || selected().to_string()
            on:mouseenter=on_mouse_enter
            on:mouseleave=on_mouse_leave
        >
            {children(cx)}
        </li>
    }
}
