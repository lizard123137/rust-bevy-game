use bevy::prelude::*;

use crate::{
    GameState,
    gameplay::{
        player::Frog,
        spatial_hash_grid::{
            CELL_SIZE,
            SpatialHashGrid,
        },
    },
};

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            check_player_distance
                .run_if(in_state(GameState::Game)),
            check_dialogue_timer,

        ));
    }
}

#[derive(Debug, Component)]
pub struct Dialogue {
    pub sentences: Vec<String>,
    pub current: usize,
    pub timer: Timer,
    pub active: bool,
} 

#[derive(Debug, Component)]
pub struct DialogueText;

fn check_player_distance(
    frogs: Query<&Frog>,
    grid: Res<SpatialHashGrid>,
    mut query: Query<(Entity, &Transform, &mut Dialogue), With<Dialogue>>,
    mut commands: Commands,
) {
    for (dialogue_entity, dialogue_t, mut dialogue_component) in &mut query {
        let entities_in_range = grid.get_entities_in_range(
            dialogue_t.translation.truncate(),
            Vec2::new(16.0, 16.0), // TODO refactor out the size of a single sprite for all occurences
            CELL_SIZE / 4.0,
        );

        for e in entities_in_range {
            // If the entity in range is a frog
            if frogs.get(e).is_ok() {
                // Start showing the dialogue sentences
                if !dialogue_component.active {
                    dialogue_component.active = true;
                    dialogue_component.current = 0;
                    dialogue_component.timer.reset();

                    // Spawn the dialogue text as a child of the dialogue entity
                    commands.entity(dialogue_entity).with_children(|parent| {
                        parent.spawn((
                            Text2d::new(dialogue_component.sentences[0].clone()),
                            TextFont {
                                font_size: FontSize::Px(8.0),
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            TextShadow::default(),

                            // TODO fix text placement
                            // TODO offset text by half its width to center above dialogue entity
                            Transform::from_xyz(
                                dialogue_t.translation.x - 64.0,
                                dialogue_t.translation.y,
                                10.0, // TODO refactor out the different Z indexes
                            ),

                            DialogueText,
                        ));
                    });
                }
            }
        }
    }
}

fn check_dialogue_timer(
    time: Res<Time>,
    mut query: Query<(&Children, &mut Dialogue), With<Dialogue>>,
    mut text_query: Query<&mut Text2d>,
    mut commands: Commands,
) {
    for (children, mut dialogue) in &mut query {
        if dialogue.active {
            dialogue.timer.tick(time.delta());

            if dialogue.timer.is_finished() {
                dialogue.current += 1;
                
                if dialogue.current >= dialogue.sentences.len() {
                    dialogue.current = 0;
                    dialogue.active = false;
                    
                    for child in children.iter() {
                        commands.entity(child).despawn();
                    }
                } else {
                    // Update the text
                    for child in children.iter() {
                        if let Ok(mut text) = text_query.get_mut(child) {
                            text.0 = dialogue.sentences[dialogue.current].clone();
                        }
                    }

                    dialogue.timer.reset();
                }
            }
        }
    }
}