use bevy::prelude::*;

pub fn c_centers(node: &mut NodeBundle) {
    node.style.display = Display::Flex;
    node.style.align_content = AlignContent::SpaceAround;
    node.style.justify_content = JustifyContent::SpaceAround;
    node.style.align_items = AlignItems::Center;
    node.style.justify_items = JustifyItems::Center;
}

pub fn c_background(node: &mut NodeBundle) {
    node.background_color = Color::DARK_GREEN.into();
    node.style.height = Val::Percent(100.0);
    node.style.width = Val::Percent(100.0);
}

pub fn c_button(_: &AssetServer, node: &mut ButtonBundle) {
    node.style.padding = UiRect::all(Val::Px(8.0));
}

pub fn c_button_text(_b: &AssetServer, a: &mut TextStyle) {
    a.color = Color::BLACK;
}
