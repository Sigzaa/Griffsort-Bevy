use bevy::prelude::*;
use super::resources::*;
use corgee::GameState;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn startup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
    mut windows: ResMut<Windows>
){

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("pepe.png"),
        ..default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
    // Text with one section
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                // position: Rect {
                //     bottom: Val::Px(10.),
                //     right: Val::Px(10.),
                //     ..default()
                // },
                
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Griffsort",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(MainMenuLabel)
        .insert(MainMenu);
        commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .insert(MainMenuButton)
        .insert(MainMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..default()
            });
        });
}
pub fn label_update(
    time: Res<Time>, 
    mut query: Query<(&mut Text, &mut Style), With<MainMenuLabel>>,
    mut windows: ResMut<Windows>
) {
    let win = windows.primary_mut();
    for (mut text, mut style) in query.iter_mut() {
        let seconds = time.seconds_since_startup() as f32;
        // We used the `Text::with_section` helper method, but it is still just a `Text`,
        // so to update it, we are still updating the one and only section
        // text.sections[0].style.color = Color::Rgba {
        //     red: (1.25 * seconds).sin() / 2.0 + 0.5,
        //     green: (0.75 * seconds).sin() / 2.0 + 0.5,
        //     blue: (0.50 * seconds).sin() / 2.0 + 0.5,
        //     alpha: 1.0,
        // };
        // style.position = Rect {
        //     bottom: Val::Px(win.height() * 0.8),
        //     right: Val::Px((win.width() / 2. )),
        //     ..default()
        // };
        
    }
}
pub fn cleanup(
    mut query: Query<Entity, With<MainMenu>>,
    mut commands: Commands,
){
    for entity in query.iter(){
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<GameState>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Play".to_string();
                *color = PRESSED_BUTTON.into();
                state.set(GameState::InGame);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Play".to_string();
                *color = NORMAL_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Play".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}