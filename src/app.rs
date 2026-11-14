use crate::{
    components::{PageLink, PagePanel},
    pages::{about::About, home::Home, projects::Projects, resume::Resume},
    routing::{Page, current_hash},
};
use leptos::{ev::hashchange, prelude::*};

#[component]
pub(crate) fn App() -> impl IntoView {
    let hash = RwSignal::new(current_hash());
    let page = Signal::derive(move || Page::from_hash(&hash.get()));
    let listener = window_event_listener(hashchange, move |_| {
        hash.set(current_hash());
        window().scroll_to_with_x_and_y(0.0, 0.0);
    });
    on_cleanup(move || listener.remove());

    view! {
        <header class="site-header page-width">
            <a class="site-name" href="#home">"Jun Yu Jian"</a>
            <nav aria-label="Main navigation">
                <ul class="topnav" data-active=move || (page.get() as u8).to_string()>
                    <PageLink page=page destination=Page::Home href="#home" label="Home"/>
                    <PageLink page=page destination=Page::Projects href="#projects" label="Projects"/>
                    <PageLink page=page destination=Page::Resume href="#resume" label="Résumé"/>
                    <PageLink page=page destination=Page::About href="#about" label="About"/>
                </ul>
            </nav>
        </header>

        <main class="page-viewport">
            <PagePanel page=page destination=Page::Home labelledby="home-heading">
                <Home/>
            </PagePanel>

            <PagePanel page=page destination=Page::Projects labelledby="projects-heading">
                <Projects page=page hash=hash/>
            </PagePanel>

            <PagePanel page=page destination=Page::Resume labelledby="resume-heading">
                <Resume/>
            </PagePanel>

            <PagePanel page=page destination=Page::About labelledby="about-heading">
                <About/>
            </PagePanel>
        </main>

        <footer class="site-footer page-width">
            <p>"Author: Jun Yu Jian"</p>
            <a class="email-link" href="mailto:luisjan0411@gmail.com">"luisjan0411@gmail.com"</a>
        </footer>
    }
}
