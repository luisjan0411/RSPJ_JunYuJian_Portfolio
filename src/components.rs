use crate::routing::Page;
use leptos::prelude::*;

#[component]
pub(crate) fn PageLink(
    page: Signal<Page>,
    destination: Page,
    href: &'static str,
    label: &'static str,
) -> impl IntoView {
    view! {
        <li>
            <a href=href aria-current=move || (page.get() == destination).then_some("page")>
                {label}
            </a>
        </li>
    }
}

#[component]
pub(crate) fn PagePanel(
    page: Signal<Page>,
    destination: Page,
    labelledby: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            class="page-panel"
            aria-labelledby=labelledby
            hidden=move || page.get() != destination
        >
            <div class="page-width page-content">{children()}</div>
        </section>
    }
}
