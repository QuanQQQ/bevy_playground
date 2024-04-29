use bevy::prelude::*;

use crate::{states::MyAppState, LoadingProcess};
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadingProcess::default())
            .add_systems(Startup, cal_load)
            .add_systems(Update, load.run_if(in_state(MyAppState::LoadingScreen)));
    }
}

fn cal_load(mut loading_process: ResMut<LoadingProcess>) {
    loading_process.total = 100;
}

fn load(
    mut loading_process: ResMut<LoadingProcess>,
    mut next_state: ResMut<NextState<MyAppState>>,
) {
    loading_process.done += 1;
    dbg!(&loading_process);
    if loading_process.done == loading_process.total {
        next_state.set(MyAppState::MainMenu)
    }
}
