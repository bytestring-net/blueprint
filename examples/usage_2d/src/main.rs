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
fn setup(mut commands: Commands) {

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

        let root = UiLink::path("Root");
        ui.spawn((
            EditorUi,
            root.clone(),
            UiLayout::Window::FULL.pack(),
            //Transform::default(),
            //Dimension::default(),
            //Background { color: Color::rgb_u8(48, 52, 70) },
        ));

        ui.spawn((
            EditorUi,
            root.add("SidePanel"),
            UiLayout::Window::new().size(Abs(Vec2::new(60.0, 0.0)) + Prc(Vec2::new(0.0, 100.0))).pack(),
            Transform::default(),
            Dimension::default(),
            Background { color: Color::rgb_u8(35, 38, 52) },
        ));

        ui.spawn((
            EditorUi,
            root.add("Explorer"),
            UiLayout::Window::new().pos(Abs(Vec2::new(60.0, 0.0))).size(Abs(Vec2::new(280.0, 0.0)) + Prc(Vec2::new(0.0, 100.0))).pack(),
            Transform::default(),
            Dimension::default(),
            Background { color: Color::rgb_u8(41, 44, 60) },
        ));

        let window = root.add("Window");
        ui.spawn((
            EditorUi,
            window.clone(),
            UiLayout::Window::new().pos(Abs(Vec2::new(340.0, 0.0))).size(Prc(Vec2::new(100.0, 100.0)) - Abs(Vec2::new(340.0, 0.0))).pack(),
            Transform::default(),
            Dimension::default(),
            Background { color: Color::rgb_u8(48, 52, 70) },
        ));

        ui.spawn((
            EditorUi,
            window.add("Tabs"),
            UiLayout::Window::new().size(Prc(Vec2::new(100.0, 0.0)) + Abs(Vec2::new(0.0, 45.0))).pack(),
            Transform::default(),
            Dimension::default(),
            Background { color: Color::rgb_u8(35, 38, 52) },
        ));

        /* ui.spawn((
            EditorUi,
            root.add("Center"),
            UiLayout::Solid::new().size((500.0, 500.0)).align_x(Align::END).pack(),
            Transform::default(),
            Dimension::default(),
            Background { color: Color::GRAY },
            //Rounded { radius: Vec4::splat(50.0) },
            Border { color: Color::MIDNIGHT_BLUE * 2.2, thickness: Vec4::new(10.0, 10.0, 10.0, 10.0) },
        )); */

    });

}




/*#[derive(Component)]
struct RenderContainer {
    color: Color,
    corner_radii: Vec4
}
fn render_update (mut painter: ShapePainter, query: Query<(&Dimension, &RenderContainer)>) {
    for (dimension, color) in &query {

        //painter.set_translation(transform.translation);
        painter.set_scale(Vec3::splat(1.0));

        painter.color = color.color;
        painter.thickness = 1.0;
        painter.corner_radii = color.corner_radii;
        painter.rectangle(Vec2::new(dimension.size.x, dimension.size.y));
    }
}*/