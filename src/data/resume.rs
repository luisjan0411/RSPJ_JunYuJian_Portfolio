pub(crate) const RESUME_PDF: &str = "public/resume.pdf";

pub(crate) struct ResumeProject {
    pub(crate) title: &'static str,
    pub(crate) href: Option<&'static str>,
    pub(crate) details: &'static str,
    pub(crate) bullets: &'static [&'static str],
}

// Résumé contributions are curated separately from gallery descriptions.
pub(crate) const RESUME_PROJECTS: [ResumeProject; 5] = [
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
