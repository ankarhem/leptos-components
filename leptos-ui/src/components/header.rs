use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="firefox:bg-opacity-90 sticky inset-x-0 top-0 z-10 border-b border-gray-800 bg-gray-900 bg-opacity-50 py-4 backdrop-blur backdrop-filter">
            <a
                href="#main-content"
                class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-full p-1 opacity-0 transition focus:translate-y-0 focus:opacity-100 focus:ease-in"
            >
                "Jump to Content"
            </a>
            <div class="max-w-8xl mx-auto w-full px-4 sm:px-6 lg:px-8">
                <nav aria-label="Site Navigation" class="flex items-center">
                    <a
                        class="py-2 px-4 text-slate-100 hover:text-slate-400 mr-auto block active"
                        aria-current="page"
                        href="/"
                    >
                        "leptos-a11y"
                    </a>
                    <a
                        class="py-2 px-4 text-slate-100 hover:text-slate-400 hidden md:block inactive"
                        href="/design-philosophy"
                    >
                        "Design Philosophy"
                    </a>
                    <a
                        class="py-2 px-4 text-slate-100 hover:text-slate-400 hidden md:block inactive"
                        href="/labeling-and-descriptions"
                    >
                        "Labeling & Descriptions"
                    </a>
                    <a
                        // github.com/ankarhem/leptos-components"
                        href="https://github.com/ankarhem/leptos-components"
                        class="block fill-slate-100 hover:fill-slate-400"
                    >
                        <span class="sr-only">"GitHub Repository for leptos-a11y"</span>
                        <svg
                            height="32"
                            aria-hidden="true"
                            viewBox="0 0 16 16"
                            version="1.1"
                            width="32"
                            data-view-component="true"
                            class="octicon octicon-mark-github v-align-middle color-fg-default"
                        >
                            <path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"></path>
                        </svg>
                    </a>
                </nav>
            </div>
        </header>
    }
}
