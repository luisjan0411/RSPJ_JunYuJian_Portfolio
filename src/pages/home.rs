use leptos::prelude::*;

#[component]
pub(crate) fn Home() -> impl IntoView {
    view! {
        <p class="eyebrow">"Computer Science and Game Design"</p>
        <h1 id="home-heading">"Jun Yu Jian Portfolio"</h1>
        <p class="role">"Gameplay & Engine Programmer"</p>
        <p>"C++ · Unreal Engine · Unity · Rust · Blender"</p>
    }
}
