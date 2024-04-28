use bevy::prelude::*;


// #==================#
// #=== COMPONENTS ===#

/// Specifies what color to use to fill the Node
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct Background {
    pub color: Color,
}

/// Specifies what corner rounding to use for the Node
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct Rounded {
    pub radius: Vec4,
}

/// Specifies the border color and thickness of the Node
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct Border {
    pub color: Color,
    pub thickness: Vec4,
}


// #===============#
// #=== BUNDLES ===#

/* #[derive(Bundle, Debug, Clone, PartialEq)]
pub struct BlueprintUiBundle {
    pub transform: Transform,
    pub dimension: Dimension,
    pub element: Element,
}
impl Default for BlueprintUiBundle {
    fn default() -> Self {
        BlueprintUiBundle {
            transform: Transform::default(),
            dimension: Dimension::default(),
            element: Element,
        }
    }
} */