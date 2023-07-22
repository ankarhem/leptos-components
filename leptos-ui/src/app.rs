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
                <div class="relative h-[250px] w-full max-w-md text-gray-900">
                <Listbox>
                    <ListboxButton class="flex w-full cursor-default items-center justify-between gap-3 rounded-lg bg-white py-2 px-3 shadow-md focus:outline-none focus-visible:border-indigo-500 focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75 focus-visible:ring-offset-2 focus-visible:ring-offset-fuchsia-300">
                        "toggle"
                    </ListboxButton>
                    <ListboxOptions clone:options class="absolute mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 shadow-lg">
                        {options.clone().into_iter().map(|option| {
                            view! { cx,
                                <ListboxOption class="group flex cursor-default select-none items-center gap-3 py-2 px-4 focus:outline-none data-[active=true]:text-fuchsia-900 data-[active=true]:bg-fuchsia-100 aria-selected:font-semibold">
                                    <span class="w-5 group-aria-[selected=true]:visible invisible">
                                        "✅"
                                    </span>
                                    {option}
                                </ListboxOption>
                            }
                        }).collect::<Vec<_>>()}
                    </ListboxOptions>
                </Listbox>
                </div>
            </main>
        </Router>
    }
}
