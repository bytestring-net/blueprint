use bevy::prelude::*;
use bevy_svg::prelude::*;
use bevy_lunex::prelude::*;
use blueprint_ui::prelude::*;

// Define the grouping marker component
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct EditorUi;

// Create application
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin::<NoData, NoData, EditorUi>::new()))
        //.add_plugins(UiDebugPlugin::<NoData, NoData, EditorUi>::new())
        .add_plugins(BlueprintUiPlugin::<NoData, NoData, EditorUi>::new())
        .add_plugins(SvgPlugin)
        .add_systems(Startup, setup)
        .run();
}

// Declare layout and logic
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    // Load font handles
    let font_light: Handle<Font> = asset_server.load("fonts/Roboto/Roboto-Light.ttf");
    let font_medium: Handle<Font> = asset_server.load("fonts/Roboto/Roboto-Medium.ttf");
    //let font_bold: Handle<Font> = asset_server.load("fonts/Roboto/Roboto-Bold.ttf");

    // Spawn camera as source for the ui
    commands.spawn(( EditorUi, Camera2dBundle { transform: Transform::from_xyz(0.0, 0.0, 1000.0), ..default() } ));


    // Spawn the main ui entity
    commands.spawn((
        UiTreeBundle::<NoData, NoData, EditorUi> {
            tree: UiTree::new("Editor"),
            ..default()
        },
        MovableByCamera,
    )).with_children(|ui| {

        // Spawn our root node
        let root = UiLink::path("Root");
        ui.spawn(UiNodeBundle::<EditorUi> {
            link: root.clone(),
            layout: UiLayout::Window::FULL.pack(),
            ..default()
        });

        // Spawn side panel node
        let panel = root.add("SidePanel");
        ui.spawn((
            UiNodeBundle::<EditorUi> {
                link: panel.clone(),
                layout: UiLayout::Window::new().size(Abs(Vec2::new(60.0, 0.0)) + Prc(Vec2::new(0.0, 100.0))).pack(),
                ..default()
            },
            UiElementBundle::default(),
            Background { color: Color::rgb_u8(35, 38, 52) },
        ));

            ui.spawn((
                UiNodeBundle::<EditorUi> {
                    link: panel.add("Files"),
                    layout: UiLayout::Window::new().size(Abs(Vec2::new(0.0, 60.0)) + Prc(Vec2::new(100.0, 0.0))).pack(),
                    ..default()
                },
                Element::default(),
                Svg2dBundle {
                    svg: asset_server.load("icons/files.svg"),
                    origin: Origin::Center,
                    transform: Transform::from_scale(Vec3::ONE * 1.5),
                    ..Default::default()
                }
            ));
            ui.spawn((
                UiNodeBundle::<EditorUi> {
                    link: panel.add("Git"),
                    layout: UiLayout::Window::new().pos((0.0, 60.0)).size(Abs(Vec2::new(0.0, 60.0)) + Prc(Vec2::new(100.0, 0.0))).pack(),
                    ..default()
                },
                Element::default(),
                Svg2dBundle {
                    svg: asset_server.load("icons/git-compare.svg"),
                    origin: Origin::Center,
                    transform: Transform::from_scale(Vec3::ONE * 1.5),
                    ..Default::default()
                }
            ));
            ui.spawn((
                UiNodeBundle::<EditorUi> {
                    link: panel.add("Extensions"),
                    layout: UiLayout::Window::new().pos((0.0, 120.0)).size(Abs(Vec2::new(0.0, 60.0)) + Prc(Vec2::new(100.0, 0.0))).pack(),
                    ..default()
                },
                Element::default(),
                Svg2dBundle {
                    svg: asset_server.load("icons/layout-grid.svg"),
                    origin: Origin::Center,
                    transform: Transform::from_scale(Vec3::ONE * 1.5),
                    ..Default::default()
                }
            ));

        // Spawn explorer node
        let explorer = root.add("Explorer");
        ui.spawn((
            UiNodeBundle::<EditorUi> {
                link: explorer.clone(),
                layout: UiLayout::Window::new().pos(Abs(Vec2::new(60.0, 0.0))).size(Abs(Vec2::new(280.0, 0.0)) + Prc(Vec2::new(0.0, 100.0))).pack(),
                ..default()
            },
            UiElementBundle::default(),
            Background { color: Color::rgb_u8(41, 44, 60) },
        ));

            let entry = explorer.add("Root Folder");
            ui.spawn((
                UiNodeBundle::<EditorUi> {
                    link: entry.clone(),
                    layout: UiLayout::Window::new().pos(Abs(Vec2::new(0.0, 20.0)) + Prc(Vec2::new(5.0, 0.0))).size(Abs(Vec2::new(0.0, 20.0)) + Prc(Vec2::new(95.0, 0.0))).pack(),
                    ..default()
                },
            ));
                ui.spawn((
                    UiNodeBundle::<EditorUi> {
                        link: entry.add("Icon"),
                        layout: UiLayout::Solid::new().align_x(-1.0).pack(),
                        ..default()
                    },
                    Element::default(),
                    Svg2dBundle {
                        svg: asset_server.load("icons/folder-open.svg"),
                        origin: Origin::Center,
                        transform: Transform::from_scale(Vec3::ONE * 1.0),
                        ..Default::default()
                    },
                ));
                ui.spawn((
                    UiNodeBundle::<EditorUi> {
                        link: entry.add("TextB"),
                        layout: UiLayout::Window::new().pos((30.0, 0.0)).size(Prc(Vec2::new(80.0, 100.0))).pack(),
                        ..default()
                    },
                ));
                    ui.spawn((
                        UiNodeBundle::<EditorUi> {
                            link: entry.add("TextB/Text"),
                            layout: UiLayout::Solid::new().align_x(-1.0).pack(),
                            ..default()
                        },
                        Background { color: Color::rgb_u8(41, 44, 60) },
                        UiText2dBundle {
                            text: Text::from_section("BLUEPRINT_UI", TextStyle {
                                font: font_medium.clone(),
                                font_size: 50.0,
                                color: Color::rgb_u8(166, 209, 137),
                            }),
                            ..default()
                        }
                    ));





        // Spawn window node
        let window = root.add("Window");
        ui.spawn((
            UiNodeBundle::<EditorUi> {
                link: window.clone(),
                layout: UiLayout::Window::new().pos(Abs(Vec2::new(340.0, 0.0))).size(Prc(Vec2::new(100.0, 100.0)) - Abs(Vec2::new(340.0, 0.0))).pack(),
                ..default()
            },
            UiElementBundle::default(),
            Background { color: Color::rgb_u8(48, 52, 70) },
        ));

        // Spawn active tabs node
        let tabs = window.add("Tabs");
        ui.spawn((
            UiNodeBundle::<EditorUi> {
                link: tabs.clone(),
                layout: UiLayout::Window::new().size(Prc(Vec2::new(100.0, 0.0)) + Abs(Vec2::new(0.0, 45.0))).pack(),
                ..default()
            },
            UiElementBundle::default(),
            Background { color: Color::rgb_u8(35, 38, 52) },
        ));

            // Spawn main.rs node
            ui.spawn((
                UiNodeBundle::<EditorUi> {
                    link: tabs.add("main.rs"),
                    layout: UiLayout::Window::new().size(Prc(Vec2::new(0.0, 100.0)) + Abs(Vec2::new(150.0, 1.0))).pack(),
                    ..default()
                },
                //BlueprintUiBundle::default(),
                Background { color: Color::rgb_u8(48, 52, 70) },
                UiText2dBundle {
                    text: Text::from_section("main.rs", TextStyle {
                        font: font_light.clone(),
                        font_size: 50.0,
                        color: Color::rgb_u8(166, 209, 137),
                    }),
                    ..default()
                }
            ));

            // Spawn structs.rs node
            ui.spawn((
                UiNodeBundle::<EditorUi> {
                    link: tabs.add("structs.rs"),
                    layout: UiLayout::Window::new().pos(Abs(Vec2::new(150.0, 0.0))).size(Prc(Vec2::new(0.0, 100.0)) + Abs(Vec2::new(150.0, 1.0))).pack(),
                    ..default()
                },
                UiElementBundle::default(),
                Background { color: Color::rgb_u8(41, 44, 60) },
                /* UiText2dBundle {
                    text: Text::from_section("structs.rs", TextStyle {
                        font: font_light.clone(),
                        font_size: 50.0,
                        color: Color::rgb_u8(166, 209, 137),
                    }),
                    ..default()
                } */
            ));

            // Spawn structs.rs node
            ui.spawn((
                UiNodeBundle::<EditorUi> {
                    link: tabs.add("systems.rs"),
                    layout: UiLayout::Window::new().pos(Abs(Vec2::new(300.0, 0.0))).size(Prc(Vec2::new(0.0, 100.0)) + Abs(Vec2::new(150.0, 1.0))).pack(),
                    ..default()
                },
                Background { color: Color::rgb_u8(41, 44, 60) },
                UiText2dBundle {
                    text: Text::from_section("systems.rs", TextStyle {
                        font: font_light.clone(),
                        font_size: 50.0,
                        color: Color::rgb_u8(166, 209, 137),
                    }),
                    ..default()
                }
            ));

    });

}

