use bevy::prelude::*;
use bevy_lunex::prelude::*;

// Define the grouping marker component
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct EditorUi;

// Create application
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin::<NoData, NoData, EditorUi>::new()))
        .add_plugins(UiDebugPlugin::<NoData, NoData, EditorUi>::new())
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
        ));

        ui.spawn((
            EditorUi,
            root.add("Background"),
            UiLayout::Solid::new().size(Abs((1920.0, 1080.0))).cover(Cover::Full).pack(),
        ));

        ui.spawn((
            EditorUi,
            root.add("Center"),
            UiLayout::Solid::new().size(Abs((807.0, 1432.0))).pack(),
        ));

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