use bevy::app::App;
use oxidized_pixel_dungeon::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
