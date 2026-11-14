use leptos::prelude::*;

#[component]
pub(crate) fn About() -> impl IntoView {
    view! {
        <h1 id="about-heading">"About"</h1>
        <div class="about-content">
            <p>"I'm Jun Yu Jian, a gameplay and engine programmer studying Computer Science and Game Design at DigiPen Institute of Technology. I expect to graduate in December 2027."</p>
            <p>"My work focuses on the systems behind games: rendering, enemy AI, procedural generation, and editor tools. Through school team projects and personal engines, I've worked with C++, Rust, OpenGL, Unreal Engine 5, and Unity."</p>
            <p>"In team projects, my contributions include sprite and text rendering, fog and lighting effects, light and shadow detection, and spider enemy AI. My personal projects explore engine development and graphics programming, including a Rust 2D engine with scene-editing tools and a DirectX 12 renderer in development."</p>
            <div class="about-links">
                <a class="button" href="https://github.com/luisjan0411" target="_blank" rel="noopener noreferrer">"GitHub ↗"</a>
                <a class="button" href="mailto:luisjan0411@gmail.com">"Email me"</a>
                <a class="button" href="#resume">"View résumé"</a>
            </div>
        </div>
    }
}
