use components::accordion::*;
use components::button::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main class="flex flex-col items-center">
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(cx, 0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    let accordion_class = "w-[300px]".to_string();
    let item_class = "border-b".to_string();

    let trigger_class = "flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180".to_string();

    let content_classes = "overflow-hidden text-sm transition-all data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down".to_string();

    view! { cx,
        <h1 class="text-3xl my-8">"Welcome to Leptos!"</h1>
        <Accordion class=accordion_class>
            <AccordionItem
                class=item_class.clone()
                clone:content_classes
                clone:trigger_class
            >
                <AccordionTrigger
                    class=trigger_class.clone()
                >
                    "First"
                </AccordionTrigger>
                <AccordionContent
                    class=content_classes.clone()
                >
                    <div class="pb-4 pt-0">"First Content"</div>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem class=item_class.clone() clone:content_classes>
                <AccordionTrigger
                    class=trigger_class.clone()
                >
                    "Second"
                </AccordionTrigger>
                <AccordionContent
                    class=content_classes.clone()
                >
                    <div class="pb-4 pt-0">"Second Content"</div>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
