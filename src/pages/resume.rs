use crate::data::resume::{RESUME_PDF, RESUME_PROJECTS};
use leptos::prelude::*;

#[component]
pub(crate) fn Resume() -> impl IntoView {
    view! {
        <div class="resume-actions">
            <a class="button" href=RESUME_PDF download="Jun-Yu-Jian-Resume.pdf">"Download résumé PDF"</a>
        </div>
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
