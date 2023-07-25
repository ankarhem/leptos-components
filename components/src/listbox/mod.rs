use leptos::ev::Event;
use leptos::html::Ul;
use leptos::*;
use leptos_use::on_click_outside;
use uuid::Uuid;
use wasm_bindgen::JsCast;

mod types;
pub use types::*;

#[component]
pub fn Listbox<T>(
    cx: Scope,
    children: ChildrenFn,
    value: RwSignal<T>,
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView
where
    T: 'static + Clone + Copy + PartialEq,
{
    let value = ListboxValue(value);
    provide_context(cx, value);
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
        if context.open.get_untracked() {
            context.open.set(false);
            context.active.set(None);
        } else {
            context.open.set(true);
        }
    };

    view! { cx,
        <button
            id=move|| context.button_id.to_string()
            class=class
            on:click=on_click
            aria-haspopup="true"
            aria-expanded=move || open().to_string()
            aria-controls=move || context.id.to_string()
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
    let el = create_node_ref::<Ul>(cx);
    let context = use_context::<ListboxContext>(cx).unwrap();

    let active = move || context.active.get().map(|id| id.to_string());
    let open = move || context.open.get();

    let class = class.map(|c| c.into_attribute_boxed(cx));

    let button_id = move || context.button_id.to_string();

    // Handle clicking outside to close
    create_effect(cx, move |_| {
        on_click_outside(cx, el, move |event: Event| {
            if context.open.get_untracked() {
                context.open.set(false);
                context.active.set(None);

                // Prevent re-opening the listbox when clicking the button
                let element: web_sys::HtmlElement = event
                    .target()
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlElement>();
                if element.id() == button_id() {
                    event.stop_propagation();
                }
            }
        })
    });

    view! { cx,
        <Show
            when=open
            fallback=|_| ()
            clone:el
        >
            <ul
                id=move || context.id.to_string()
                _ref=el
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
pub fn ListboxOption<T>(
    cx: Scope,
    children: ChildrenFn,
    value: T,
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView
where
    T: 'static + Clone + Copy + PartialEq,
{
    let id = Uuid::new_v4();
    let context = use_context::<ListboxContext>(cx).unwrap();
    let listbox_value = use_context::<ListboxValue<T>>(cx).unwrap().0;

    let selected = move || listbox_value.get() == value;

    let on_click = move |_| {
        if !selected() {
            listbox_value.set(value);
            context.open.set(false);
            context.active.set(None);
        }
    };

    let active = move || context.active.get() == Some(id);

    if context.active.get_untracked().is_none() && listbox_value.get_untracked() == value {
        context.active.set(Some(id));
    }

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
            id=move || id.to_string()
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
