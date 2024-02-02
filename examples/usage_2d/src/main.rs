use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin::<NoData, NoData, MyWidget>::new()))
        .add_plugins(UiDebugPlugin::<NoData, NoData, MyWidget>::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {

    commands.spawn(( MyWidget, Camera2dBundle { transform: Transform::from_xyz(0.0, 0.0, 1000.0), ..default() } ));

    commands.spawn((
        UiTreeBundle::<NoData, NoData, MyWidget> {
            tree: UiTree::new("Editor"),
            ..default()
        },
        MovableByCamera,
    )).with_children(|parent| {

        let root = UiLink::path("Root");
        parent.spawn((
            MyWidget,
            root.clone(),
            UiLayout::Window::FULL.pack(),
        ));


        parent.spawn((
            MyWidget,
            UiLink::path("Root/Square"),
            UiLayout::Solid::new().size(Abs((1920.0, 1080.0))).cover(Cover::Full).pack(),
        ));

        parent.spawn((
            MyWidget,
            UiLink::path("Root/Board"),
            UiLayout::Solid::new().size(Abs((807.0, 1432.0))).align_x(Align(-0.8)).pack(),
        ));

    });

}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;



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