use bevy::window::PrimaryWindow;
use bevy::prelude::*;

// Too lazy to comment everything

#[derive(Debug, Resource)]
/// Cursor Tools. Only for primary Window
pub struct Cursor {
    pub position: Option<Vec2>,
    pub valid_position: Vec2,
    window_scale: Vec2,
    last_position: Option<Vec2>,
}
impl Default for Cursor {
    fn default() -> Self {
        Self { position: None, valid_position: Vec2::ZERO, window_scale: Vec2::ZERO, last_position: None }
    }
}
impl Cursor {
    fn convert_to_translation(position: Vec2, window_scale: Vec2) -> Vec2 {
        Vec2::new(position.x - window_scale.x / 2.0, -(position.y - window_scale.y / 2.0))
    }
    pub fn translation(&self) -> Option<Vec2> {
        return if let Some(position) = self.position {
            Some(Self::convert_to_translation(position, self.window_scale))
        } else {
            None
        }
    }
    pub fn valid_translation(&self) -> Vec2 {
        Self::convert_to_translation(self.valid_position, self.window_scale)
    }
    pub fn window_scale(&self) -> Vec2 {
        self.window_scale
    }
    pub fn cursor_velocity(&self) -> Option<Vec2> {
        Some(self.position? - self.last_position?)
    }
}

pub struct CursorResourcePlugin;
impl Plugin for CursorResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Cursor::default())
            .add_systems(PreUpdate, cursor)
        ;
    }
}

fn cursor(
    query_windows: Query<&Window, With<PrimaryWindow>>,
    mut cursor: ResMut<Cursor>,
) {
    if let Ok(window) = query_windows.get_single() {
        cursor.last_position = cursor.position;
        cursor.position = window.cursor_position();
        cursor.window_scale = Vec2::new(window.width(), window.height());
        if let Some(position) = window.cursor_position() {
            cursor.valid_position = position
        }
    }
}