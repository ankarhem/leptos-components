use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod components;
mod routes;
use components::*;
use routes::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <Header/>
            <div class="mx-auto flex max-w-8xl px-4 sm:px-6 lg:px-8">
                <Navigation/>
                <main id="main-content" class="flex min-w-0 flex-1 pt-4">
                    <article class="mx-auto min-w-0 flex-1 prose prose-invert prose-headings:scroll-mt-24 lg:prose-lg min-w-0 max-w-none">
                    <Routes>
                        <Route path="/components/listbox" view=ListboxPage />
                    </Routes>
                    </article>
                <div class="hidden w-64 shrink-0 lg:block lg:pl-8">
                    <p id="toc" class="font-semibold uppercase tracking-wide text-white">"Table of Contents"</p>
                    <nav aria-labelledby="toc">
                        <ul>
                        </ul>
                    </nav>
                </div>
                </main>
            </div>
        </Router>
    }
}

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use leptos::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}
