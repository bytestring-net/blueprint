use bevy::prelude::*;


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
