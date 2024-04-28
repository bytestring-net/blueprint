#![doc = include_str!("../../../README.md")]


mod preset;
mod theme;

use bevy::prelude::*;
use bevy_lunex::prelude::*;
pub use bevy_vector_shapes::prelude::*;

pub mod prelude {
    pub use super::BlueprintUiPlugin;
    pub use super::Background;
}


#[derive(Component)]
pub struct Background {
    pub color: Color,
}

fn render_update(mut painter: ShapePainter,
    camera: Query<&Transform, (With<Camera>, Without<Background>)>,
    query: Query<(&Transform, &Dimension, &Background)>
) {
    let camera = camera.single();
    //painter.set_translation(camera.translation);

    for (transform, dimension, color) in &query {
        info!("Rendering {}", transform.translation);

        painter.set_translation(camera.translation + transform.translation/2.0);
        painter.set_scale(Vec3::splat(1.0));

        painter.color = color.color;
        painter.thickness = 1.0;
        //painter.corner_radii = color.corner_radii;
        painter.rect(dimension.size);
    }
}



pub fn render_ui<M:Default + Component, N:Default + Component, T: Component>(
    uis: Query<(&UiTree<M, N>, &Children), (With<T>, Changed<UiTree<M, N>>)>,
    mut query: Query<(&UiLink, &mut Transform), (With<T>, With<Element>)>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut transform)) = query.get_mut(*child) {
                /* // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        transform.translation = container.rectangle.pos.invert_y();
                        transform.translation.x += container.rectangle.size.x /  2.0;
                        transform.translation.y += container.rectangle.size.y / -2.0;
                    }
                } */
            }
        }
    }
}


pub struct BlueprintUiPlugin;
impl Plugin for BlueprintUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Shape2dPlugin::default())
            .add_systems(Update, render_update);
    }
}