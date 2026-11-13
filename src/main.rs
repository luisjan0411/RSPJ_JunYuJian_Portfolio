use leptos::{ev::hashchange, prelude::*};

const RESUME_PDF: &str = "/public/resume.pdf";

fn main() {
    mount_to_body(App);
}

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    Projects,
    Resume,
    About,
}

impl Page {
    fn from_hash(hash: &str) -> Self {
        match hash {
            "#projects" | "#school_projects" | "#personal_projects" => Self::Projects,
            _ if Project::from_hash(hash).is_some() => Self::Projects,
            "#about" => Self::About,
            "#resume" => Self::Resume,
            _ => Self::Home,
        }
    }
}

#[derive(Clone, Copy)]
enum ProjectMedia {
    Image {
        src: &'static str,
        alt: &'static str,
    },
    Video(&'static str),
    Placeholder,
}

struct Project {
    href: &'static str,
    title: &'static str,
    category: &'static str,
    tools: &'static str,
    description: &'static str,
    media: ProjectMedia,
    link: Option<&'static str>,
}

const PROJECTS: [Project; 3] = [
    Project {
        href: "#projects/dungeon-generation",
        title: "Procedural Dungeon Generation in Unity",
        category: "School Project",
        tools: "Unity",
        description: "A procedural dungeon generator in Unity. The map shows the generated rooms and corridors.",
        media: ProjectMedia::Image {
            src: "/public/images/PCGDungeonMap.png",
            alt: "Procedurally generated dungeon map",
        },
        link: None,
    },
    Project {
        href: "#projects/climbing-ai",
        title: "StateTree, AI Perception, and Climbing AI",
        category: "School Project",
        tools: "Unreal Engine 5",
        description: "An Unreal Engine 5 project focused on StateTree, AI Perception, and climbing behavior.",
        media: ProjectMedia::Placeholder,
        link: Some("https://www.youtube.com/"),
    },
    Project {
        href: "#projects/custom-2d-engine",
        title: "Custom 2D Game Engine",
        category: "Personal Project",
        tools: "Rust · OpenGL",
        description: "A custom 2D game engine written in Rust using OpenGL. The demo shows the engine rendering a pixel art scene.",
        media: ProjectMedia::Video("/public/videos/Custom2DEnginePixelArtVideo.mp4"),
        link: None,
    },
];

impl Project {
    fn from_hash(hash: &str) -> Option<&'static Self> {
        PROJECTS.iter().find(|project| project.href == hash)
    }
}

fn current_hash() -> String {
    window().location().hash().unwrap_or_default()
}

#[component]
fn App() -> impl IntoView {
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
                <p class="eyebrow">"Computer Science and Game Design"</p>
                <h1 id="home-heading">"Jun Yu Jian Portfolio"</h1>
                <p class="role">"Gameplay & Engine Programmer"</p>
                <p>"C++ · Unreal Engine · Unity · Rust · Blender"</p>
            </PagePanel>

            <PagePanel page=page destination=Page::Projects labelledby="projects-heading">
                {move || {
                    if page.get() != Page::Projects {
                        return ().into_any();
                    }

                    match Project::from_hash(&hash.get()) {
                        Some(project) => view! { <ProjectDetail project=project/> }.into_any(),
                        None => view! {
                            <h1 id="projects-heading">"Projects"</h1>
                            <ul class="project-gallery">
                                {PROJECTS.iter().map(|project| view! {
                                    <li><ProjectCard project=project/></li>
                                }).collect_view()}
                            </ul>
                        }.into_any(),
                    }
                }}
            </PagePanel>

            <PagePanel page=page destination=Page::Resume labelledby="resume-heading">
                <h1 id="resume-heading">"Résumé"</h1>
                <p class="role">"Gameplay & Engine Programmer"</p>
                <p>"Computer Science and Game Design"</p>
                <div class="resume-document">
                    <a href=RESUME_PDF target="_blank" rel="noopener">"Open résumé template PDF"</a>
                    <iframe class="resume-pdf" src=RESUME_PDF title="Jun Yu Jian résumé template"></iframe>
                </div>
                <h2 class="resume-heading">"Technical Skills"</h2>
                <p>"C++ · Unreal Engine · Unity · Rust · Blender · OpenGL"</p>
                <h2 class="resume-heading">"Selected Projects"</h2>
                <ul>
                    {PROJECTS.iter().map(|project| view! { <li><a href=project.href>{project.title}</a></li> }).collect_view()}
                </ul>
            </PagePanel>

            <PagePanel page=page destination=Page::About labelledby="about-heading">
                <h1 id="about-heading">"About"</h1>
                <p>"This portfolio is made with Rust and Trunk."</p>
            </PagePanel>
        </main>

        <footer class="site-footer page-width">
            <p>"Author: Jun Yu Jian"</p>
            <a class="email-link" href="mailto:luisjan0411@gmail.com">"luisjan0411@gmail.com"</a>
        </footer>
    }
}

#[component]
fn PageLink(
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
fn PagePanel(
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

#[component]
fn ProjectCard(project: &'static Project) -> impl IntoView {
    let autoplay = window()
        .match_media("(prefers-reduced-motion: no-preference)")
        .ok()
        .flatten()
        .is_some_and(|query| query.matches());

    view! {
        <a class="project-card" href=project.href>
            <div class="project-preview" aria-hidden="true">
                {match project.media {
                    ProjectMedia::Image { src, .. } => view! {
                        <img src=src alt="" loading="lazy"/>
                    }.into_any(),
                    ProjectMedia::Video(src) => view! {
                        <video src=src autoplay=autoplay muted loop playsinline preload="metadata"></video>
                    }.into_any(),
                    ProjectMedia::Placeholder => view! { <span>"No preview"</span> }.into_any(),
                }}
            </div>
            <div class="project-card-copy">
                <p class="eyebrow">{project.category}" · "{project.tools}</p>
                <h2>{project.title}</h2>
            </div>
        </a>
    }
}

#[component]
fn ProjectDetail(project: &'static Project) -> impl IntoView {
    view! {
        <article class="project-detail">
            <a class="project-back" href="#projects">
                <span aria-hidden="true">"←"</span>
                "All projects"
            </a>
            <p class="eyebrow">{project.category}" · "{project.tools}</p>
            <h1 id="projects-heading">{project.title}</h1>
            <p>{project.description}</p>
            {match project.media {
                ProjectMedia::Image { src, alt } => view! {
                    <img class="project-detail-media" src=src alt=alt/>
                }.into_any(),
                ProjectMedia::Video(src) => view! {
                    <video class="project-detail-media" src=src controls playsinline preload="metadata"
                        aria-label="Custom 2D game engine demo">
                        "Your browser does not support video."
                    </video>
                }.into_any(),
                ProjectMedia::Placeholder => ().into_any(),
            }}
            {project.link.map(|href| view! { <a class="button" href=href>"YouTube ↗"</a> })}
        </article>
    }
}
