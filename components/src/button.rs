use leptos::*;

#[component]
pub fn Button(cx: Scope, #[prop(optional)] children: Option<Children>) -> impl IntoView {
    let children = children.map(|c| c(cx));

    view! { cx,
        <button class="test">
            {children}
        </button>
    }
}
