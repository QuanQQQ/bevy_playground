use crate::*;

pub struct CustomDebugPlugin;
impl Plugin for CustomDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, represent_towards);
    }
}
pub fn represent_towards(
    query: Query<(&Towards, &Children), (Changed<Towards>)>,
    mut text: Query<&mut Text>,
) {
    for (toward, children) in query.iter() {
        for &child in children {
            // let mut child_text = text.get_mut(child).expect("Children should have text");
            match text.get_mut(child) {
                Ok(mut child_text) => {
                    *child_text = Text::from_section(
                        match toward {
                            Towards::Down => "down",
                            Towards::Up => "up",
                            Towards::Left => "left",
                            Towards::Right => "right",
                        },
                        TextStyle::default(),
                    );
                }
                _ => {}
            }
        }
    }
}
