use leptos::*;
use leptos_router::*;

struct NavItem {
    title: String,
    route: String,
}

struct Navigation {
    items: Vec<NavItem>,
}

impl Navigation {
    fn new() -> Self {
        Self { items: vec![] }
    }

    fn add<S>(mut self, route: S, title: S) -> Self
    where
        S: Into<String>,
    {
        self.items.push(NavItem {
            title: title.into(),
            route: route.into(),
        });
        self
    }
}

#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
    let navigation = Navigation::new()
        .add("/components/menu", "Menu (Dropdown)")
        .add("/components/listbox", "Listbox (Select)")
        .add("/components/combobox", "Combobox (Autocomplete)")
        .add("/components/switch", "Switch (Toggle)")
        .add("/components/disclosure", "Disclosure")
        .add("/components/dialog", "Dialog (Modal)")
        .add("/components/popover", "Popover")
        .add("/components/radio-group", "Radio Group")
        .add("/components/tabs", "Tabs");

    view! { cx,
        <div class="hidden w-64 shrink-0 lg:block lg:pr-8">
            <div class="sticky top-[100px]">
                <p id="components-nav-header" class="mb-4 font-semibold uppercase tracking-wide text-white text-lg">
                    "Components"
                </p>
                <nav class="mx-auto space-y-4 text-gray-400" aria-labelledby="components-nav-header">
                    <ul>
                        {navigation
                            .items
                            .into_iter()
                            .map(|item| {
                                view! { cx,
                                    <li>
                                        <A
                                            href=item.route
                                            class="transition flex items-center whitespace-nowrap px-2 font-medium no-underline aria-[current=page]:text-white hover:text-white"
                                        >
                                            {item.title}
                                        </A>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                </nav>
            </div>
        </div>
    }
}
