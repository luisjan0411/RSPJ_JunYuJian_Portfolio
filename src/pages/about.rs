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
                <a class="button about-button" href="https://github.com/luisjan0411" target="_blank" rel="noopener noreferrer">
                    <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true" focusable="false">
                        <path d="M12 .75a11.25 11.25 0 0 0-3.56 21.92c.56.1.77-.24.77-.54v-2.1c-3.13.68-3.79-1.33-3.79-1.33-.51-1.3-1.25-1.65-1.25-1.65-1.02-.7.08-.69.08-.69 1.13.08 1.73 1.16 1.73 1.16 1 .1 1.76 1.95 3.28 1.39.1-.73.39-1.23.71-1.51-2.5-.28-5.13-1.25-5.13-5.56 0-1.23.44-2.23 1.16-3.02-.12-.28-.5-1.43.11-2.98 0 0 .95-.3 3.09 1.15a10.76 10.76 0 0 1 5.62 0c2.14-1.45 3.08-1.15 3.08-1.15.62 1.55.23 2.7.12 2.98.72.79 1.15 1.79 1.15 3.02 0 4.32-2.63 5.27-5.14 5.55.41.35.77 1.03.77 2.08v2.66c0 .3.2.65.77.54A11.25 11.25 0 0 0 12 .75Z"/>
                    </svg>
                    <span>"GitHub"</span>
                </a>
                <a class="button about-button" href="mailto:luisjan0411@gmail.com">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">
                        <rect x="3" y="5" width="18" height="14" rx="2"/>
                        <path d="m3 6 9 7 9-7"/>
                    </svg>
                    <span>"Email me"</span>
                </a>
                <a class="button about-button" href="#resume">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">
                        <path d="M14 3H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9Z"/>
                        <path d="M14 3v6h6M8 13h8M8 17h6"/>
                    </svg>
                    <span>"View résumé"</span>
                </a>
            </div>
        </div>
    }
}
