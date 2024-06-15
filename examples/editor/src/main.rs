use bevy::prelude::*;
use bevy_svg::prelude::*;
use bevy_lunex::prelude::*;
use blueprint_ui::prelude::*;

// Create application
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiGeneralPlugin))
        //.add_plugins(UiDebugPlugin::new())
        .add_plugins(BlueprintUiPlugin::<MainUi>::new())
        .add_plugins(SvgPlugin)
        .add_systems(Startup, setup)
        .run();
}

// Declare layout and logic
fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {

    // Load font handles
    //let font_light: Handle<Font> = asset_server.load("fonts/Roboto/Roboto-Light.ttf");
    //let font_medium: Handle<Font> = asset_server.load("fonts/Roboto/Roboto-Medium.ttf");
    //let font_bold: Handle<Font> = asset_server.load("fonts/Roboto/Roboto-Bold.ttf");

    // Spawn camera as source for the ui
    commands.spawn(( MainUi, Camera2dBundle { transform: Transform::from_xyz(0.0, 0.0, 1000.0), ..default() } ));


    // Spawn the main ui entity
    commands.spawn((
        UiTreeBundle::<MainUi> {
            tree: UiTree::new("Editor"),
            ..default()
        },
        MovableByCamera,
    )).with_children(|ui| {

        ui.spawn((
            UiLink::<MainUi>::path("Root"),
            UiLayout::window_full().pack::<Base>(),
            UiElementBundle::default(),
            Background { color: Color::rgb_u8(48, 52, 70) },
        ));

        ui.spawn((
            UiLink::<MainUi>::path("Root/Bar1"),
            UiLayout::window().size((Ab(60.), Rl(100.))).pack::<Base>(),
            UiElementBundle::default(),
            Background { color: Color::rgb_u8(35, 38, 52) },
        ));

        ui.spawn((
            UiLink::<MainUi>::path("Root/Bar2"),
            UiLayout::window().x(60.0).size((300.0, Rl(100.0))).pack::<Base>(),
            UiElementBundle::default(),
            Background { color: Color::rgb_u8(41, 44, 60) },
        ));

    });
}

