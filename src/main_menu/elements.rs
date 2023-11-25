use bevy::prelude::*;

use super::styles::{BUTTON_STYLE, NORMAL_BUTTON_COLOR};

pub fn new_text_label(text: &str, style: TextStyle) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection::new(text, style)],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}

pub fn new_button() -> ButtonBundle {
    ButtonBundle {
        style: BUTTON_STYLE,
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
    }
}
