use leptos::*;
use leptos_router::*;

#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="sticky top-[98px] max-h-screen flex-shrink-0 overflow-y-auto pb-20 hidden w-48 lg:block">
            <h2>Components</h2>
            <nav>
                <ul>
                    <li><A href="/components/listbox">Listbox</A></li>
                </ul>
            </nav>
        </div>
    }
}
