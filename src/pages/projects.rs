use crate::{
    data::projects::{PROJECTS, Project, ProjectGroup, ProjectMedia},
    routing::Page,
};
use leptos::prelude::*;

#[component]
pub(crate) fn Projects(page: Signal<Page>, hash: RwSignal<String>) -> impl IntoView {
    view! {
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
