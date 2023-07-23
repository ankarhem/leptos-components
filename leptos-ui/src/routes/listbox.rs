use leptos::*;
use leptos_a11y::listbox::*;

#[component]
fn DropdownIcon(cx: Scope) -> impl IntoView {
    view! { cx,
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" class="h-5 w-5 text-gray-400"><path fill-rule="evenodd" d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z" clip-rule="evenodd"></path></svg>
    }
}

#[component]
pub fn ListboxPage(cx: Scope) -> impl IntoView {
    let options = vec!["one", "two"];
    let value = create_rw_signal(cx, options[0]);

    view! { cx,

        <h1>"Listbox (Select)"</h1>
        <p>
            "Listboxes are a great foundation for building custom, accessible select menus for your app, complete with robust support for keyboard navigation."
        </p>
        <section class="not-prose relative">
            <div class="flex h-full min-h-[180px] items-center justify-center rounded-xl bg-gradient-to-r p-12 from-fuchsia-500 to-purple-600">
                <div class="relative h-[250px] w-full max-w-md text-gray-900">
                    <Listbox value=value>
                        <ListboxButton class="flex w-full cursor-default items-center justify-between gap-3 rounded-lg bg-white py-2 px-3 shadow-md focus:outline-none focus-visible:border-indigo-500 focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75 focus-visible:ring-offset-2 focus-visible:ring-offset-fuchsia-300">
                            {value}
                            <DropdownIcon />
                        </ListboxButton>
                        <ListboxOptions
                            clone:options
                            class="absolute mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 shadow-lg"
                        >
                            {options
                                .clone()
                                .into_iter()
                                .map(|option| {
                                    view! { cx,
                                        <ListboxOption value=option class="group flex cursor-default select-none items-center gap-3 py-2 px-4 focus:outline-none data-[active=true]:text-fuchsia-900 data-[active=true]:bg-fuchsia-100 aria-[selected=true]:font-semibold">
                                            <span class="w-5 group-aria-[selected=true]:visible invisible">
                                                "✅"
                                            </span>
                                            {option}
                                        </ListboxOption>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </ListboxOptions>
                    </Listbox>
                </div>
            </div>
        </section>
    }
}
