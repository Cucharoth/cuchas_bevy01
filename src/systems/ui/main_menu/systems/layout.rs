use crate::systems::ui::main_menu::systems::style::*;
use crate::{main, systems::ui::main_menu::systems::components::*};
use bevy::prelude::*;
use bevy_ui_navigation::{
    prelude::{DefaultNavigationPlugins, FocusState, Focusable, NavEvent, NavRequestSystem},
    systems::InputMapping,
};


pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, mut imput_mapping: ResMut<InputMapping>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            create_child_half_screen_left(parent, &asset_server);
            create_child_half_screen_right(parent, &asset_server, imput_mapping);
        });
}

fn create_child_half_screen_left(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) -> Entity {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(60.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: MAIN_MENU_BACKGROUND_COLOR.into(),
            ..Default::default()
        })
        .with_children(
            |parent| {
                parent.spawn(
                    ImageBundle {
                        style: Style {
                            width: Val::Percent(70.0),
                            height: Val::Percent(80.0),
                            margin: UiRect::new(Val::Px(200.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
                            ..Default::default()
                        },
                        image: asset_server.load("Spirit.png").into(),
                        ..Default::default()
                    }
                );
            }
        )
        .id()
}

fn create_child_half_screen_right(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    mut imput_mapping: ResMut<InputMapping>
) -> Entity {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(40.0),
                height: Val::Percent(100.0),
                column_gap: Val::Px(8.0),
                row_gap: Val::Px(8.0),
                ..Default::default()
            },
            background_color: MAIN_MENU_BACKGROUND_COLOR.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            create_title_node(parent, asset_server);
            create_play_button(parent, asset_server, &mut imput_mapping);
            create_exit_button(parent, asset_server, &mut imput_mapping);
        })
        .id()
}

fn create_title_node(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) -> Entity {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(400.0),
                height: Val::Px(120.0),
                ..Default::default()
            },
            background_color: MAIN_MENU_BACKGROUND_COLOR.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
                    ..Default::default()
                },
                image: asset_server.load("logo.png").into(),
                ..Default::default()
            });
            parent.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Cucha's Rpg",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: MAIN_MENU_BUTTON_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Left,
                    ..Default::default()
                },
                ..Default::default()
            
            },
        )
        );
        })
        .id()
}

fn create_play_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, imput_mapping: &mut ResMut<InputMapping>) -> Entity {
    imput_mapping.keyboard_navigation = true;
    imput_mapping.focus_follows_mouse = true;
    
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: MAIN_MENU_BUTTON_WIDTH,
                    height: MAIN_MENU_BUTTON_HEIGHT,
                    ..Default::default()
                },
                background_color: MAIN_MENU_BACKGROUND_COLOR.into(),
                ..Default::default()
            },
            PlayButton,
        ))
        .with_children(|current_parent| {
            current_parent.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: MAIN_MENU_BUTTON_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Left,
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenuPlayText,
            Focusable::default()
        ));
        })
        .id()
}

fn create_exit_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, imput_mapping: &mut ResMut<InputMapping>) -> Entity {
    imput_mapping.keyboard_navigation = true;
    imput_mapping.focus_follows_mouse = true;

    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: MAIN_MENU_BUTTON_WIDTH,
                    height: MAIN_MENU_BUTTON_HEIGHT,
                    ..Default::default()
                },
                background_color: MAIN_MENU_BACKGROUND_COLOR.into(),
                ..Default::default()
            },
            ExitButton,
        ))
        .with_children(|current_parent| {
            current_parent.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Exit",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: MAIN_MENU_BUTTON_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Left,
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenuExitText,
            Focusable::default()
        )
        );
        })
        .id()
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}
