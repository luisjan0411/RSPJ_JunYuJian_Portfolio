use leptos::{ev::hashchange, prelude::*};

const RESUME_PDF: &str = "public/resume.pdf";

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
    Video(&'static str),
}

#[derive(Clone, Copy, PartialEq)]
enum ProjectGroup {
    School,
    Personal,
}

struct Project {
    group: ProjectGroup,
    href: &'static str,
    title: &'static str,
    category: &'static str,
    tools: &'static str,
    description: &'static str,
    authorship: &'static str,
    role: &'static str,
    contributions: &'static [&'static str],
    breakdown: &'static [(&'static str, &'static str)],
    showcase_note: &'static str,
    next_steps: Option<&'static str>,
    media: ProjectMedia,
    media_heading: Option<&'static str>,
    detail_images: &'static [(&'static str, &'static str, &'static str)],
    gameplay_video: Option<&'static str>,
    link: Option<&'static str>,
}

const PROJECTS: [Project; 7] = [
    Project {
        href: "#projects/dx12-renderer",
        authorship: "Solo project",
        role: "Graphics programmer",
        contributions: &[
            "Built the current renderer prototype in C++ using DirectX 12.",
            "Rendered a rotating 3D plane with per-vertex color.",
        ],
        breakdown: &[
            ("3D rendering prototype", "The current implementation demonstrates a rotating plane in 3D. This is an early renderer prototype, with the showcase focused on the functionality already implemented."),
            ("Per-vertex color", "Color is assigned at the vertex level. The plane demonstrates how those colors appear across the rendered surface as it rotates."),
        ],
        showcase_note: "Watch the plane rotate and the color transition across its surface. This video shows the current development milestone.",
        next_steps: None,
        title: "C++ DirectX 12 Renderer",
        group: ProjectGroup::Personal,
        category: "Personal Project · In Development",
        tools: "C++ · DirectX 12",
        description: "A renderer in development using C++ and DirectX 12. The current demo renders a rotating 3D plane with per-vertex color.",
        media: ProjectMedia::Video("public/videos/C++DX12RendererShowcase.mp4"),
        media_heading: Some("Renderer Showcase"),
        detail_images: &[],
        gameplay_video: None,
        link: None,
    },
    Project {
        href: "#projects/team-top-down-shooter",
        authorship: "Team project — Team Ransomware",
        role: "Engine programmer — rendering, serialization, entities and systems",
        contributions: &[
            "Implemented sprite and text rendering, including Y- and Z-based sorting, in the team's custom 2D engine.",
            "Created the game's fog and lighting effects.",
            "Contributed to part of the mesh-rendering implementation.",
            "Implemented JSON serialization for scene, entity, and sprite information.",
            "Contributed entity and system programming within the engine.",
        ],
        breakdown: &[
            ("Sprite and text rendering", "I implemented sprite and text rendering in the team's C++ and OpenGL engine, including Y- and Z-based sorting to control draw order."),
            ("Fog and lighting", "I created the game's fog and lighting effects. The gameplay showcase presents these effects alongside the sprite rendering in the complete team project."),
            ("Mesh rendering", "I contributed to part of the mesh-rendering implementation; this was a shared system rather than one I developed entirely on my own."),
            ("Scene serialization", "The serialization work stores scene, entity, and sprite information in JSON. These are the core data categories I handled for representing the game's scene content."),
            ("Team contribution", "My responsibilities were engine-side rendering, serialization, and entity/system programming. The game shown in the video is the combined work of Team Ransomware."),
        ],
        showcase_note: "The video shows the team's game in action; my contributions focus on the engine systems supporting the scene and sprites.",
        next_steps: None,
        title: "Bubbly Wubbly",
        category: "School Team Project · Team Ransomware",
        group: ProjectGroup::School,
        tools: "C++ · OpenGL · JSON",
        description: "Bubbly Wubbly is a top-down shooter made by Team Ransomware using a custom 2D engine in C++ and OpenGL. My work included sprite and text rendering, Y- and Z-based sorting, fog and lighting effects, JSON serialization, and entity and system programming. I also contributed to part of the mesh-rendering implementation.",
        media: ProjectMedia::Video("public/videos/BubblyWubbly.mp4"),
        media_heading: Some("Gameplay Showcase"),
        detail_images: &[],
        gameplay_video: None,
        link: None,
    },
    Project {
        href: "#projects/dungeon-generation",
        authorship: "Solo project",
        role: "Procedural generation programmer and pixel artist",
        contributions: &[
            "Built randomized hub generation in Unity.",
            "Implemented A* pathfinding to connect hubs with corridors.",
            "Created custom pixel art for the dungeon.",
        ],
        breakdown: &[
            ("Randomized hubs", "Generation begins with randomized hubs. These hubs provide the locations that the corridor-generation logic connects."),
            ("Corridor connections", "I wrote the A* pathfinding implementation used to connect hubs with corridors. The map shows the resulting relationship between the hub areas and their connecting paths."),
            ("Custom pixel art", "I created the dungeon's pixel art as part of the same solo project, combining procedural generation with the visual presentation of its output."),
        ],
        showcase_note: "Watch the dungeon showcase, then inspect the map below for the generated hubs and connecting corridors.",
        next_steps: None,
        title: "Procedural Dungeon Generation in Unity",
        category: "School Project",
        group: ProjectGroup::School,
        tools: "Unity",
        description: "A procedural dungeon generator in Unity that creates randomized hubs and connects them with corridors using my own A* pathfinding implementation. The showcase video also features custom pixel art I created for the dungeon.",
        media: ProjectMedia::Video("public/videos/PCGDungeonVideo.mp4"),
        gameplay_video: None,
        media_heading: Some("Dungeon Showcase"),
        detail_images: &[
            ("Generated Dungeon Map", "public/images/PCGDungeonShowcase.png", "Generated dungeon with hub areas connected by corridors and custom pixel art"),
        ],
        link: None,
    },
    Project {
        href: "#projects/climbing-ai",
        authorship: "Team project — Team Cryptid Craft",
        role: "Enemy AI developer",
        contributions: &[
            "Developed the spider enemy AI as my contribution to Team Cryptid Craft's ongoing horror game in Unreal Engine 5.",
            "Used AI Perception and StateTree to organize enemy behavior and combat decisions.",
            "Structured the StateTree around normal behavior, combat behavior, and reactions.",
        ],
        breakdown: &[
            ("Perception and state selection", "AI Perception is used alongside StateTree for the enemy's behavior logic. In the StateTree shown below, player visibility is a condition for entering combat behavior and selecting attacks."),
            ("Normal and combat behavior", "The tree groups search, calm, and patrol under normal behavior. Combat decisions include repositioning, attack selection, spawning children, and chasing."),
            ("Attack conditions and reactions", "The attack-selection branch contains bite, claw, lunge, and area attacks with distance conditions. A separate reaction branch handles knockback. The diagram exposes these decisions alongside the gameplay demonstration."),
        ],
        showcase_note: "These showcases focus on the spider enemy I worked on within Team Cryptid Craft's ongoing horror game. Compare the gameplay with the StateTree diagram to see the enemy's behavior logic.",
        next_steps: None,
        title: "StateTree, AI Perception, and Climbing AI",
        category: "School Team Project · In Development",
        group: ProjectGroup::School,
        tools: "Unreal Engine 5",
        description: "My spider enemy AI contribution to an ongoing horror game developed by Team Cryptid Craft in Unreal Engine 5. My work focuses on AI Perception, StateTree behavior, and climbing AI within the larger team project.",
        media: ProjectMedia::Video("public/videos/SpiderEnemyModelShowcase0001-0300.mp4"),
        gameplay_video: Some("public/videos/SpiderEnemyShowcase.mp4"),
        media_heading: Some("Model Showcase"),
        detail_images: &[
            ("StateTree Behavior", "public/images/SpiderAIStateTree.png", "Spider AI StateTree showing normal behavior, combat decisions, attack selection, and knockback reactions"),
        ],
        link: None,
    },
    Project {
        href: "#projects/custom-2d-engine",
        authorship: "Solo project",
        role: "Engine and editor programmer",
        contributions: &[
            "Developed a custom 2D engine in Rust using OpenGL.",
            "Built an egui interface for scene editing and object manipulation.",
            "Applied overlay and generated noise textures in world space when rendering the tilemap.",
        ],
        breakdown: &[
            ("Rendering", "The engine uses Rust and OpenGL to render 2D content. The demonstration shows the current engine rendering a pixel-art scene."),
            ("World-space tilemap textures", "The tilemap rendering uses an overlay texture and a generated noise texture applied in world space. The texture mapping is based on world-space coordinates across the tilemap."),
            ("Editor tools", "The engine includes an egui-based scene editor for editing scenes and adjusting objects. The updated showcase demonstrates the scene editor and object-editing workflow alongside the rendered scene."),
        ],
        showcase_note: "Watch the egui scene editor and object-editing workflow alongside the engine's rendered pixel-art scene.",
        next_steps: Some("Add a debug console for displaying messages. This is planned functionality and has not been implemented yet."),
        title: "Custom 2D Game Engine",
        category: "Personal Project",
        group: ProjectGroup::Personal,
        tools: "Rust · OpenGL · egui",
        description: "A custom 2D game engine written in Rust using OpenGL, with an egui-based scene editor. The new showcase demonstrates scene editing and object manipulation alongside a rendered pixel-art scene.",
        media: ProjectMedia::Video("public/videos/Custom2DengineShowcase.mp4"),
        media_heading: None,
        detail_images: &[],
        gameplay_video: None,
        link: None,
    },
    Project {
        href: "#projects/abandoned-sky-island",
        authorship: "Solo project",
        role: "Level designer",
        contributions: &[
            "Created the abandoned sky island level as an individual project.",
            "Developed the level layout and environment composition shown in the showcase.",
        ],
        breakdown: &[
            ("Level layout", "The overhead image presents the arrangement of buildings, terrain, and connecting bridges. It provides a broader view of the environment than the ground-level showcase."),
            ("Environment views", "The vista shows the island's silhouette, trees, and a bridge leading toward a separate floating island. The video and still images show the same environment from different viewpoints."),
        ],
        showcase_note: "Use the video for a ground-level view, the overview to inspect the layout, and the vista to see the island's larger composition.",
        next_steps: None,
        title: "Abandoned Sky Island",
        category: "School Project",
        group: ProjectGroup::School,
        tools: "Level Design",
        description: "A level design project set on an abandoned sky island. Explore the environment through a video showcase, an overhead view of the level, and a vista of the island.",
        media: ProjectMedia::Video("public/videos/LevelDesignOverview.mp4"),
        media_heading: Some("Level Showcase"),
        detail_images: &[
            ("Level Overview", "public/images/SkyIslandOverview.png", "Overhead view of the sky island's buildings, terrain, and connecting bridges"),
            ("Vista", "public/images/SkyIslandVista.png", "View across the island toward trees and a bridge leading to a floating island"),
        ],
        gameplay_video: None,
        link: None,
    },
    Project {
        href: "#projects/lightreading",
        authorship: "Team project",
        role: "Light and shadow programmer",
        contributions: &[
            "Implemented light and shadow detection using raycasting in the DGL engine.",
            "Developed hard shadows for the game's light-and-shadow system.",
        ],
        breakdown: &[
            ("Raycasting detection", "My detection work uses raycasting to determine light and shadow conditions. This is the detection side of my contribution to the team's light-and-shadow system."),
            ("Hard shadows", "I implemented the shadow portion using hard shadows, with sharp boundaries between illuminated and shadowed areas. The showcase presents this work within the team's top-down game."),
            ("Contribution scope", "LightReading was made as a team project with the DGL engine. My responsibilities were light/shadow detection and the hard-shadow implementation."),
        ],
        showcase_note: "Watch the illuminated and shadowed areas in the scene; these are the focus of my contribution to the team project.",
        next_steps: None,
        title: "LightReading",
        category: "School Team Project",
        group: ProjectGroup::School,
        tools: "DGL Engine · Raycasting · Hard Shadows",
        description: "LightReading is a team-developed top-down game made with the DGL engine. I implemented light and shadow detection using raycasting and developed the hard-shadow portion of the project.",
        media: ProjectMedia::Video("public/videos/LightReadingOverview.mp4"),
        media_heading: Some("Gameplay Showcase"),
        detail_images: &[],
        gameplay_video: None,
        link: None,
    },
];

struct ResumeProject {
    title: &'static str,
    href: Option<&'static str>,
    details: &'static str,
    bullets: &'static [&'static str],
}

// Résumé contributions are curated separately from gallery descriptions.
const RESUME_PROJECTS: [ResumeProject; 5] = [
    ResumeProject {
        title: "LightReading",
        href: Some("#projects/lightreading"),
        details: "School Team Project | DGL Engine",
        bullets: &[
            "Implemented light and shadow detection using raycasting.",
            "Developed hard shadows for the game's light-and-shadow system.",
        ],
    },
    ResumeProject {
        title: "Bubbly Wubbly",
        href: Some("#projects/team-top-down-shooter"),
        details: "School Team Project | Team Ransomware | C++, OpenGL, JSON",
        bullets: &[
            "Implemented sprite and text rendering with Y- and Z-based sorting in the team's custom 2D engine.",
            "Created fog and lighting effects for the game.",
            "Implemented JSON serialization for scene, entity, and sprite data.",
        ],
    },
    ResumeProject {
        title: "Custom 2D Game Engine",
        href: Some("#projects/custom-2d-engine"),
        details: "Personal Project | Rust, OpenGL, egui",
        bullets: &[
            "Developed a custom 2D game engine using Rust and OpenGL.",
            "Built an egui-based editor for scene editing and object manipulation.",
        ],
    },
    ResumeProject {
        title: "Spider AI",
        href: Some("#projects/climbing-ai"),
        details: "Team Cryptid Craft | In Development | Unreal Engine 5",
        bullets: &[
            "Developed spider enemy AI for the team's ongoing horror game using AI Perception and StateTree for behavior and combat decisions.",
        ],
    },
    ResumeProject {
        title: "Procedural Dungeon Generation",
        href: Some("#projects/dungeon-generation"),
        details: "School Project | Unity, A* Pathfinding, Pixel Art",
        bullets: &[
            "Developed a dungeon generator that creates randomized hubs and connects them with corridors using a self-implemented A* pathfinding algorithm.",
            "Created custom pixel art for the dungeon.",
        ],
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
                            {[
                                (ProjectGroup::School, "school-projects-heading", "School Projects"),
                                (ProjectGroup::Personal, "personal-projects-heading", "Personal Projects"),
                            ].into_iter().map(|(group, heading_id, title)| view! {
                                <section class="project-group" aria-labelledby=heading_id>
                                    <h2 id=heading_id>{title}</h2>
                                    <ul class="project-gallery">
                                        {PROJECTS.iter().filter(move |project| project.group == group).map(|project| view! {
                                            <li><ProjectCard project=project/></li>
                                        }).collect_view()}
                                    </ul>
                                </section>
                            }).collect_view()}
                        }.into_any(),
                    }
                }}
            </PagePanel>

            <PagePanel page=page destination=Page::Resume labelledby="resume-heading">
                <div class="resume-actions">
                    <a class="button" href=RESUME_PDF download="Jun-Yu-Jian-Resume.pdf">"Download résumé PDF"</a>
                </div>
                <Resume/>
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
fn Resume() -> impl IntoView {
    view! {
        <article class="resume-sheet" aria-label="Jun Yu Jian résumé">
            <header class="resume-banner">
                <h1 id="resume-heading">"Jun Yu Jian"</h1>
                <p class="resume-title">"Gameplay & Engine Programmer"</p>
                <a href="mailto:luisjan0411@gmail.com">"luisjan0411@gmail.com"</a>
            </header>
            <div class="resume-body">
                <section class="resume-section" aria-labelledby="resume-summary">
                    <h2 id="resume-summary">"Summary"</h2>
                    <p>"Computer Science and Game Design student focused on gameplay and engine programming, with project experience in rendering, enemy AI, procedural generation, and editor tools."</p>
                </section>
                <section class="resume-section" aria-labelledby="resume-skills">
                    <h2 id="resume-skills">"Skills"</h2>
                    <ul class="resume-skills">
                        <li>"C++"</li>
                        <li>"Rust"</li>
                        <li>"Unreal Engine 5"</li>
                        <li>"Unity"</li>
                        <li>"OpenGL"</li>
                        <li>"Blender"</li>
                    </ul>
                </section>
                <section class="resume-section" aria-labelledby="resume-projects">
                    <h2 id="resume-projects">"Project Experience"</h2>
                    {RESUME_PROJECTS.iter().map(|project| view! {
                        <div class="resume-entry">
                            <h3>{match project.href {
                                Some(href) => view! { <a href=href>{project.title}</a> }.into_any(),
                                None => project.title.into_any(),
                            }}</h3>
                            <p class="resume-entry-meta">{project.details}</p>
                            <ul>{project.bullets.iter().map(|&bullet| view! {
                                <li>{bullet}</li>
                            }).collect_view()}</ul>
                        </div>
                    }).collect_view()}
                </section>
                <section class="resume-section" aria-labelledby="resume-education">
                    <h2 id="resume-education">"Education and Training"</h2>
                    <div class="resume-entry">
                        <h3>"DigiPen Institute of Technology"</h3>
                        <p>"Bachelor of Science in Computer Science and Game Design"</p>
                        <p>"Expected Graduation: December 2027"</p>
                    </div>
                </section>
            </div>
        </article>
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
                    ProjectMedia::Video(src) => view! {
                        <video src=src autoplay=autoplay muted prop:muted=true loop playsinline preload="metadata"></video>
                    }.into_any(),
                }}
            </div>
            <div class="project-card-copy">
                <p class="eyebrow">{project.category}{(!project.tools.is_empty()).then(|| format!(" · {}", project.tools))}</p>
                <h3>{project.title}</h3>
            </div>
        </a>
    }
}

#[component]
fn ProjectDetail(project: &'static Project) -> impl IntoView {
    let autoplay = window()
        .match_media("(prefers-reduced-motion: no-preference)")
        .ok()
        .flatten()
        .is_some_and(|query| query.matches());

    view! {
        <article class="project-detail">
            <a class="project-back" href="#projects">
                <span aria-hidden="true">"←"</span>
                "All projects"
            </a>
            <p class="eyebrow">{project.category}{(!project.tools.is_empty()).then(|| format!(" · {}", project.tools))}</p>
            <h1 id="projects-heading">{project.title}</h1>
            <section class="project-writeup" aria-labelledby="project-overview-heading">
                <h2 id="project-overview-heading">"Project Overview"</h2>
                <p>{project.description}</p>
            </section>
            <section class="project-writeup" aria-labelledby="project-info-heading">
                <h2 id="project-info-heading">"Project Info"</h2>
                <dl class="project-facts">
                    <div><dt>"Development"</dt><dd>{project.authorship}</dd></div>
                    <div><dt>"My Role"</dt><dd>{project.role}</dd></div>
                    <div><dt>"Tools & Focus"</dt><dd>{project.tools}</dd></div>
                    {project.category.contains("In Development").then(|| view! {
                        <div><dt>"Status"</dt><dd>"In Development"</dd></div>
                    })}
                </dl>
            </section>
            <p class="project-showcase-note">{project.showcase_note}</p>
            {project.media_heading.map(|heading| view! {
                <h2 class="project-showcase-heading">{heading}</h2>
            })}
            {match project.media {
                ProjectMedia::Video(src) => view! {
                    <video class="project-detail-media" src=src autoplay=autoplay muted prop:muted=true loop playsinline preload="metadata"
                        aria-label=project.title>
                        "Your browser does not support video."
                    </video>
                }.into_any(),
            }}
            {project.gameplay_video.map(|src| view! {
                <h2 class="project-showcase-heading">"Gameplay Showcase"</h2>
                <video class="project-detail-media" src=src autoplay=autoplay muted prop:muted=true loop playsinline preload="metadata"
                    aria-label=format!("{} gameplay showcase", project.title)>
                    "Your browser does not support video."
                </video>
            })}
            {project.detail_images.iter().map(|&(heading, src, alt)| view! {
                <h2 class="project-showcase-heading">{heading}</h2>
                <img class="project-detail-media" src=src alt=alt loading="lazy"/>
            }).collect_view()}
            <section class="project-writeup" aria-labelledby="project-contributions-heading">
                <h2 id="project-contributions-heading">"My Contributions"</h2>
                <ul>{project.contributions.iter().map(|&item| view! {
                    <li>{item}</li>
                }).collect_view()}</ul>
            </section>
            <section class="project-writeup" aria-labelledby="project-breakdown-heading">
                <h2 id="project-breakdown-heading">"Technical Breakdown"</h2>
                {project.breakdown.iter().map(|&(title, description)| view! {
                    <div class="project-breakdown-item">
                        <h3>{title}</h3>
                        <p>{description}</p>
                    </div>
                }).collect_view()}
            </section>
            {project.next_steps.map(|text| view! {
                <section class="project-writeup" aria-labelledby="project-next-heading">
                    <h2 id="project-next-heading">"Next Steps"</h2>
                    <p>{text}</p>
                </section>
            })}
            {project.link.map(|href| view! { <a class="button" href=href>"YouTube ↗"</a> })}
        </article>
    }
}
