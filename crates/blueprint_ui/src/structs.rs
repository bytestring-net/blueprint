use bevy::prelude::*;
use bevy_lunex::prelude::*;


// #==================#
// #=== COMPONENTS ===#

/// Specifies what color to use to fill the Node
#[derive(Component)]
pub struct Background {
    pub color: Color,
}

/// Specifies what corner rounding to use for the Node
#[derive(Component)]
pub struct Rounded {
    pub radius: Vec4,
}

/// Specifies the border color and thickness of the Node
#[derive(Component)]
pub struct Border {
    pub color: Color,
    pub thickness: Vec4,
}


// #===============#
// #=== BUNDLES ===#

/// Specifies the border color and thickness of the Node
#[derive(Bundle)]
pub struct UiNodeBundle<T: Component> {
    pub marker: T,
    pub link: UiLink,
    pub layout: Layout,
    pub transform: Transform,
    pub dimension: Dimension,
}
impl <T: Component + Default> Default for UiNodeBundle<T> {
    fn default() -> Self {
        UiNodeBundle {
            marker: T::default(),
            link: UiLink::default(),
            layout: Layout::default(),
            transform: Transform::default(),
            dimension: Dimension::default(),
        }
    }
}