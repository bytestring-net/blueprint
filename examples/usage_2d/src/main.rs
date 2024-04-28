use bevy::prelude::*;
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
        .add_systems(Startup, setup)
        .run();
}

// Declare layout and logic
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    // Load font handles
    let font_light: Handle<Font> = asset_server.load("fonts/Roboto/Roboto-Light.ttf");
    let font_medium: Handle<Font> = asset_server.load("fonts/Roboto/Roboto-Medium.ttf");
    let font_bold: Handle<Font> = asset_server.load("fonts/Roboto/Roboto-Bold.ttf");

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
        ui.spawn((
            UiNodeBundle::<EditorUi> {
                link: root.add("SidePanel"),
                layout: UiLayout::Window::new().size(Abs(Vec2::new(60.0, 0.0)) + Prc(Vec2::new(0.0, 100.0))).pack(),
                ..default()
            },
            BlueprintUiBundle::default(),
            Background { color: Color::rgb_u8(35, 38, 52) },
        ));

        // Spawn explorer node
        ui.spawn((
            UiNodeBundle::<EditorUi> {
                link: root.add("Explorer"),
                layout: UiLayout::Window::new().pos(Abs(Vec2::new(60.0, 0.0))).size(Abs(Vec2::new(280.0, 0.0)) + Prc(Vec2::new(0.0, 100.0))).pack(),
                ..default()
            },
            BlueprintUiBundle::default(),
            Background { color: Color::rgb_u8(41, 44, 60) },
        ));

        // Spawn window node
        let window = root.add("Window");
        ui.spawn((
            UiNodeBundle::<EditorUi> {
                link: window.clone(),
                layout: UiLayout::Window::new().pos(Abs(Vec2::new(340.0, 0.0))).size(Prc(Vec2::new(100.0, 100.0)) - Abs(Vec2::new(340.0, 0.0))).pack(),
                ..default()
            },
            BlueprintUiBundle::default(),
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
            BlueprintUiBundle::default(),
            Background { color: Color::rgb_u8(35, 38, 52) },
        ));

        // Spawn main.rs node
        ui.spawn((
            UiNodeBundle::<EditorUi> {
                link: tabs.add("main.rs"),
                layout: UiLayout::Window::new().size(Prc(Vec2::new(0.0, 100.0)) + Abs(Vec2::new(150.0, 1.0))).pack(),
                ..default()
            },
            BlueprintUiBundle::default(),
            Background { color: Color::rgb_u8(48, 52, 70) },
        ));

        // Spawn structs.rs node
        ui.spawn((
            UiNodeBundle::<EditorUi> {
                link: tabs.add("structs.rs"),
                layout: UiLayout::Window::new().pos(Abs(Vec2::new(150.0, 0.0))).size(Prc(Vec2::new(0.0, 100.0)) + Abs(Vec2::new(150.0, 1.0))).pack(),
                ..default()
            },
            BlueprintUiBundle::default(),
            Background { color: Color::rgb_u8(41, 44, 60) },
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
                text: Text::from_section("systems.rs",
                    TextStyle {
                        font: font_medium.clone(),
                        font_size: 50.0,
                        color: Color::rgb_u8(166, 209, 137),
                    }),
                ..default()
            }
        ));

    });

}

