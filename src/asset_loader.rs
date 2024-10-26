use bevy::prelude::*;

#[derive(Resource ,Debug, Default)]
pub struct SceneAssets {
    pub asteroids: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missile: Handle<Scene>,
    pub planet: Handle<Scene>
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
) {
    *scene_assets = SceneAssets {
        asteroids: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Planet.glb")),
        spaceship: asset_server.load("models/Spaceship.glb#Scene0"),
        missile: asset_server.load("models/Spaceship-u105mYHLHU.glb#Scene0"),
        planet: asset_server.load("models/Planet-1.glb#Scene0"),
    }
}