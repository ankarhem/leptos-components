use leptos::html::Li;
use leptos::*;
use uuid::Uuid;

mod types;
pub use types::*;

#[component]
pub fn Listbox(
    cx: Scope,
    children: ChildrenFn,
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
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
    #[prop(optional, into)] class: Option<AttributeValue>,
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
pub fn ListboxOptions(
    cx: Scope,
    children: ChildrenFn,
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
    let context = use_context::<ListboxContext>(cx).unwrap();

    let active = move || context.active.get().map(|id| id.to_string());
    let open = move || context.open.get();

    let class = class.map(|c| c.into_attribute_boxed(cx));

    view! { cx,
        <Show
            when=open
            fallback=|_| ()
        >
            <ul
                id=context.id.to_string()
                role="listbox"
                tabindex="0"
                class=class.clone()
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
    #[prop(optional, into)] class: Option<AttributeValue>,
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
            on:mouseenter=on_mouse_enter
            on:mouseleave=on_mouse_leave
            disabled=move || disabled
            data-active=move || active().to_string()
            aria-selected=move || selected().to_string()
        >
            {children(cx)}
        </li>
    }
}
