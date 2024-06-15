use std::marker::PhantomData;
use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;

use crate::structs::*;


// #===================#
// #=== CORE SYSTEM ===#

/// This system is behind the component polling and rendering the node.
pub fn render_ui<T: Component, N:Default + Component>(
    mut painter: ShapePainter,
    uis: Query<(&Children, &GlobalTransform), (With<UiLink<T>>, With<UiTree<T, N>>)>,
    query: Query<(
        &Transform,
        &Dimension,
        Option<&Background>,
        Option<&Rounded>,
        Option<&Border>,
    ), With<UiLink<T>>>,
) {
    for (children, global_transform) in &uis {
        for child in children {
            if let Ok((transform, dimension, background, rounded, border)) = query.get(*child) {

                // Apply transform
                let mut pos = transform.translation + global_transform.translation();
                pos.z -= 1.0;
                painter.set_translation(pos);
    
                // Apply background color
                if let Some(background) = background {
                    painter.color = background.color;
                }

                // Apply rounded cornes
                if let Some(rounded) = rounded {
                    painter.corner_radii = rounded.radius;
                }

                // Draw background
                painter.rect(dimension.size);

                // Apply border
                if let Some(border) = border {
                    // Condition to check if we can use simplified rectangle border or we need to draw separate lines
                    let simplified = if let Some(rounded) = rounded {
                        Vec4::ZERO != rounded.radius || rounded.radius.x == rounded.radius.y && rounded.radius.y == rounded.radius.z && rounded.radius.z == rounded.radius.w
                    } else { true };

                    painter.color = border.color;

                    if simplified {
                        painter.thickness = border.thickness.max_element();
                        painter.hollow = true;
                        painter.rect(dimension.size);
                    } else {
                        let halfsize = dimension.size/2.0;
                        painter.cap = Cap::None;

                        // Left
                        painter.thickness = border.thickness.x;
                        painter.line(Vec3::new(halfsize.x, -halfsize.y, 1.0), Vec3::new(halfsize.x, halfsize.y, 1.0));

                        // Top
                        painter.thickness = border.thickness.y;
                        painter.line(Vec3::new(-halfsize.x, halfsize.y, 1.0), Vec3::new(halfsize.x, halfsize.y, 1.0));

                        // Right
                        painter.thickness = border.thickness.z;
                        painter.line(Vec3::new(-halfsize.x, -halfsize.y, 1.0), Vec3::new(-halfsize.x, halfsize.y, 1.0));

                        // Bottom
                        painter.thickness = border.thickness.w;
                        painter.line(Vec3::new(-halfsize.x, -halfsize.y, 1.0), Vec3::new(halfsize.x, -halfsize.y, 1.0));
                    }
                }
            }
        }
    }
}


// #===============#
// #=== PLUGINS ===#

#[derive(Debug, Default, Clone)]
pub struct BlueprintUiPlugin <T: Component = MainUi, N:Default + Component = NoData>(PhantomData<T>, PhantomData<N>);
impl <T: Component, N:Default + Component> BlueprintUiPlugin<T, N> {
    pub fn new() -> Self {
        BlueprintUiPlugin::<T, N>(PhantomData, PhantomData)
    }
}
impl <T: Component, N:Default + Component> Plugin for BlueprintUiPlugin<T, N> {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Shape2dPlugin::default())
            .add_systems(Update, render_ui::<T, N>);
    }
}