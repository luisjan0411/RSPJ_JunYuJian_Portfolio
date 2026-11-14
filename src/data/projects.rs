#[derive(Clone, Copy)]
pub(crate) enum ProjectMedia {
    Video(&'static str),
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ProjectGroup {
    School,
    Personal,
}

pub(crate) struct Project {
    pub(crate) group: ProjectGroup,
    pub(crate) href: &'static str,
    pub(crate) title: &'static str,
    pub(crate) category: &'static str,
    pub(crate) tools: &'static str,
    pub(crate) description: &'static str,
    pub(crate) authorship: &'static str,
    pub(crate) role: &'static str,
    pub(crate) contributions: &'static [&'static str],
    pub(crate) breakdown: &'static [(&'static str, &'static str)],
    pub(crate) showcase_note: &'static str,
    pub(crate) next_steps: Option<&'static str>,
    pub(crate) media: ProjectMedia,
    pub(crate) media_heading: Option<&'static str>,
    pub(crate) detail_images: &'static [(&'static str, &'static str, &'static str)],
    pub(crate) gameplay_video: Option<&'static str>,
    pub(crate) link: Option<&'static str>,
}

pub(crate) const PROJECTS: [Project; 7] = [
    Project {
        href: "#projects/dx12-renderer",
        authorship: "Solo project",
        role: "Graphics programmer",
        contributions: &[
            "Built the current renderer prototype in C++ using DirectX 12.",
            "Rendered a rotating 3D plane with per-vertex color.",
        ],
        breakdown: &[
            (
                "3D rendering prototype",
                "The current implementation demonstrates a rotating plane in 3D. This is an early renderer prototype, with the showcase focused on the functionality already implemented.",
            ),
            (
                "Per-vertex color",
                "Color is assigned at the vertex level. The plane demonstrates how those colors appear across the rendered surface as it rotates.",
            ),
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
            (
                "Sprite and text rendering",
                "I implemented sprite and text rendering in the team's C++ and OpenGL engine, including Y- and Z-based sorting to control draw order.",
            ),
            (
                "Fog and lighting",
                "I created the game's fog and lighting effects. The gameplay showcase presents these effects alongside the sprite rendering in the complete team project.",
            ),
            (
                "Mesh rendering",
                "I contributed to part of the mesh-rendering implementation; this was a shared system rather than one I developed entirely on my own.",
            ),
            (
                "Scene serialization",
                "The serialization work stores scene, entity, and sprite information in JSON. These are the core data categories I handled for representing the game's scene content.",
            ),
            (
                "Team contribution",
                "My responsibilities were engine-side rendering, serialization, and entity/system programming. The game shown in the video is the combined work of Team Ransomware.",
            ),
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
            (
                "Randomized hubs",
                "Generation begins with randomized hubs. These hubs provide the locations that the corridor-generation logic connects.",
            ),
            (
                "Corridor connections",
                "I wrote the A* pathfinding implementation used to connect hubs with corridors. The map shows the resulting relationship between the hub areas and their connecting paths.",
            ),
            (
                "Custom pixel art",
                "I created the dungeon's pixel art as part of the same solo project, combining procedural generation with the visual presentation of its output.",
            ),
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
        detail_images: &[(
            "Generated Dungeon Map",
            "public/images/PCGDungeonShowcase.png",
            "Generated dungeon with hub areas connected by corridors and custom pixel art",
        )],
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
            (
                "Perception and state selection",
                "AI Perception is used alongside StateTree for the enemy's behavior logic. In the StateTree shown below, player visibility is a condition for entering combat behavior and selecting attacks.",
            ),
            (
                "Normal and combat behavior",
                "The tree groups search, calm, and patrol under normal behavior. Combat decisions include repositioning, attack selection, spawning children, and chasing.",
            ),
            (
                "Attack conditions and reactions",
                "The attack-selection branch contains bite, claw, lunge, and area attacks with distance conditions. A separate reaction branch handles knockback. The diagram exposes these decisions alongside the gameplay demonstration.",
            ),
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
        detail_images: &[(
            "StateTree Behavior",
            "public/images/SpiderAIStateTree.png",
            "Spider AI StateTree showing normal behavior, combat decisions, attack selection, and knockback reactions",
        )],
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
            (
                "Rendering",
                "The engine uses Rust and OpenGL to render 2D content. The demonstration shows the current engine rendering a pixel-art scene.",
            ),
            (
                "World-space tilemap textures",
                "The tilemap rendering uses an overlay texture and a generated noise texture applied in world space. The texture mapping is based on world-space coordinates across the tilemap.",
            ),
            (
                "Editor tools",
                "The engine includes an egui-based scene editor for editing scenes and adjusting objects. The updated showcase demonstrates the scene editor and object-editing workflow alongside the rendered scene.",
            ),
        ],
        showcase_note: "Watch the egui scene editor and object-editing workflow alongside the engine's rendered pixel-art scene.",
        next_steps: Some(
            "Add a debug console for displaying messages. This is planned functionality and has not been implemented yet.",
        ),
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
            (
                "Level layout",
                "The overhead image presents the arrangement of buildings, terrain, and connecting bridges. It provides a broader view of the environment than the ground-level showcase.",
            ),
            (
                "Environment views",
                "The vista shows the island's silhouette, trees, and a bridge leading toward a separate floating island. The video and still images show the same environment from different viewpoints.",
            ),
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
            (
                "Level Overview",
                "public/images/SkyIslandOverview.png",
                "Overhead view of the sky island's buildings, terrain, and connecting bridges",
            ),
            (
                "Vista",
                "public/images/SkyIslandVista.png",
                "View across the island toward trees and a bridge leading to a floating island",
            ),
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
            (
                "Raycasting detection",
                "My detection work uses raycasting to determine light and shadow conditions. This is the detection side of my contribution to the team's light-and-shadow system.",
            ),
            (
                "Hard shadows",
                "I implemented the shadow portion using hard shadows, with sharp boundaries between illuminated and shadowed areas. The showcase presents this work within the team's top-down game.",
            ),
            (
                "Contribution scope",
                "LightReading was made as a team project with the DGL engine. My responsibilities were light/shadow detection and the hard-shadow implementation.",
            ),
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

impl Project {
    pub(crate) fn from_hash(hash: &str) -> Option<&'static Self> {
        PROJECTS.iter().find(|project| project.href == hash)
    }
}
