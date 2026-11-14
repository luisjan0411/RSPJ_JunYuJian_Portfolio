use leptos::prelude::*;

#[component]
pub(crate) fn Home() -> impl IntoView {
    view! {
        <div class="home-hero">
            <div class="home-intro">
                <p class="eyebrow">"Computer Science and Game Design"</p>
                <h1 id="home-heading">"Jun Yu Jian Portfolio"</h1>
                <p class="role">"Gameplay & Engine Programmer"</p>
                <p>"C++ · Unreal Engine · Unity · Rust · Blender"</p>
            </div>
            // Replace the placeholder with an image when a portrait is ready.
            <div class="home-portrait" role="img" aria-label="Profile photo coming soon">
                <svg viewBox="0 0 64 64" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true" focusable="false">
                    <circle cx="32" cy="23" r="10"/>
                    <path d="M13 54v-4a19 19 0 0 1 38 0v4"/>
                </svg>
                <span>"Photo coming soon"</span>
            </div>
        </div>
        <div class="home-sections">
            <section class="home-card" aria-labelledby="home-background">
                <h2 id="home-background">"Background"</h2>
                <ul>
                    <li>"I'm Jun Yu Jian, a game developer from Taiwan studying in the United States."</li>
                    <li>"Studying Computer Science and Game Design at DigiPen Institute of Technology."</li>
                    <li>"Expected graduation: December 2027."</li>
                </ul>
            </section>
            <section class="home-card" aria-labelledby="home-programming">
                <h2 id="home-programming">"Programming & Development"</h2>
                <ul>
                    <li>"Gameplay systems, enemy AI, and procedural generation in Unreal Engine and Unity."</li>
                    <li>"Custom engine development and graphics programming with C++, Rust, and OpenGL; currently exploring DirectX 12."</li>
                    <li>"Scene-editing tools, rendering systems, and game-data serialization."</li>
                    <li>"Experience working on both team games and solo projects."</li>
                </ul>
            </section>
            <section class="home-card" aria-labelledby="home-creative">
                <h2 id="home-creative">"Art & Design"</h2>
                <ul>
                    <li>"Creating custom pixel art for games."</li>
                    <li>"Modeling 3D characters and environments."</li>
                    <li>"Designing levels and composing game environments."</li>
                </ul>
            </section>
        </div>
    }
}
