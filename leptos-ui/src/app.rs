use std::fmt;

use leptos::*;
use leptos_a11y::listbox::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Debug, Clone)]
struct MyValue {
    value: String,
}

impl fmt::Display for MyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value: {}", self.value)
    }
}

impl IntoView for MyValue {
    fn into_view(self, cx: Scope) -> View {
        view! { cx,
            <span>{self.value}</span>
        }
        .into_view(cx)
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let options = vec!["one", "two"];

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main class="w-[600px] mx-auto">
                <Listbox>
                    <ListboxButton>
                        "toggle"
                    </ListboxButton>
                    <ListboxOptions clone:options>
                        {options.clone().into_iter().map(|option| {
                            view! { cx,
                                <ListboxOption class="aria-selected:bg-slate-400".into_attribute(cx)>
                                    {option}
                                </ListboxOption>
                            }
                        }).collect::<Vec<_>>()}
                    </ListboxOptions>
                </Listbox>
            </main>
        </Router>
    }
}
