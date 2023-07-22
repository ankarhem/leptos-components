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

        <Header/>

        <Router>
            <div class="max-w-8xl mx-auto w-full px-4 sm:px-6 lg:px-8 flex items-start pt-4 pb-24 sm:pt-6 lg:pb-12 lg:space-x-8">
                <Navigation/>
                <main id="main-content" class="w-[600px] mx-auto">
                    <Routes>
                        <Route path="/components/listbox" view=ListboxPage />
                    </Routes>
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
