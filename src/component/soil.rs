use std::ops::Add;

use bevy::prelude::*;
use seldom_state::prelude::*;

#[derive(Event, Debug, Clone)]
pub enum SoilEvent {
    Reclaim(Entity),
    Unreclaim(Entity),
    Plant(Entity),
    Unplant(Entity),
}

#[derive(Bundle)]
pub struct Soil {
    pub state: StateMachine,
    pub spatial: SpatialBundle,
}

impl Default for Soil {
    fn default() -> Self {
        Self {
            state: StateMachine::default()
                .trans::<Unreclaimed, _>(reclaim, Reclaimed::default())
                .trans::<Reclaimed, _>(unreclaim, Unreclaimed)
                .trans::<Reclaimed, _>(plant, Planted { planting: None })
                .trans::<Planted, _>(
                    unplant,
                    Reclaimed {
                        watered: false,
                        fertilized: false,
                    },
                )
                .set_trans_logging(true),
            spatial: SpatialBundle::default(),
        }
    }
}

impl Soil {
    pub fn with_transform(self, trans: Transform) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(trans),
            ..self
        }
    }
}

#[derive(Component, Clone, Default, Debug)]
#[component(storage = "SparseSet")]
pub struct Unreclaimed;

#[derive(Component, Clone, Default, Debug)]
#[component(storage = "SparseSet")]
pub struct Reclaimed {
    watered: bool,
    fertilized: bool,
}

#[derive(Component, Clone, Default, Debug)]
#[component(storage = "SparseSet")]
pub struct Planted {
    planting: Option<Entity>,
}

fn reclaim(In(soil_e): In<Entity>, mut events: EventReader<SoilEvent>) -> bool {
    for event in events.read() {
        if let SoilEvent::Reclaim(entity) = event {
            return entity == &soil_e;
        }
    }
    false
}
fn unreclaim(In(soil_e): In<Entity>, mut events: EventReader<SoilEvent>) -> bool {
    for event in events.read() {
        if let SoilEvent::Unreclaim(entity) = event {
            return entity == &soil_e;
        }
    }
    false
}
fn plant(In(soil_e): In<Entity>, mut events: EventReader<SoilEvent>) -> bool {
    for event in events.read() {
        if let SoilEvent::Plant(entity) = event {
            return entity == &soil_e;
        }
    }
    false
}
fn unplant(In(soil_e): In<Entity>, mut events: EventReader<SoilEvent>) -> bool {
    for event in events.read() {
        if let SoilEvent::Unplant(entity) = event {
            return entity == &soil_e;
        }
    }
    false
}

pub struct SoilPlugin;
impl Plugin for SoilPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoilEvent>()
            .add_systems(Update, update_text);
    }
}

fn update_text(
    reclaimed: Query<&Children, Added<Reclaimed>>,
    unreclaimed: Query<&Children, Added<Unreclaimed>>,
    planted: Query<&Children, Added<Planted>>,
    mut text: Query<&mut Text>,
) {
    for children in reclaimed.iter() {
        for &child in children {
            match text.get_mut(child) {
                Ok(mut text) => {
                    *text = Text::from_section("RSoil", TextStyle::default())
                        .with_justify(JustifyText::Center);
                }
                Err(err) => {}
            }
        }
    }
    for children in unreclaimed.iter() {
        for &child in children {
            match text.get_mut(child) {
                Ok(mut text) => {
                    *text = Text::from_section("Soil", TextStyle::default())
                        .with_justify(JustifyText::Center);
                }
                Err(err) => {}
            }
        }
    }
    for children in planted.iter() {
        for &child in children {
            match text.get_mut(child) {
                Ok(mut text) => {
                    *text = Text::from_section("Planted", TextStyle::default())
                        .with_justify(JustifyText::Center);
                }
                Err(err) => {}
            }
        }
    }
}
