use crate::prelude::fight::components::*;
use crate::systems::ui::fight::systems::components::*;
use crate::systems::ui::fight::systems::style::*;
use bevy::prelude::*;

use bevy_ui_navigation::prelude::FocusAction;
use bevy_ui_navigation::prelude::FocusState;
use bevy_ui_navigation::prelude::Focusable;
use bevy_ui_navigation::prelude::MenuBuilder;
use bevy_ui_navigation::systems::InputMapping;

pub fn create_fight_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut input_mapping: ResMut<InputMapping>,
    mut enemy_q: Query<&Enemy, With<Enemy>>,
) {
    commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::End,
                //justify_content: JustifyContent::Start,
                //align_content: AlignContent::End,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        FightNodeRoot
    ))
        .with_children(|parent| {
            create_buttons_node(parent, &asset_server, &mut input_mapping);
            create_enemy_status_node(parent, &asset_server, enemy_q);
        });
}

fn create_enemy_status_node(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    mut enemy_q: Query<&Enemy, With<Enemy>>,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(20.0),
                    height: Val::Percent(12.0),
                    //top: Val::Percent(1.),
                    bottom: Val::Percent(85.),
                    left: Val::Percent(2.),
                    //right: Val::Percent(10.),
                    padding: UiRect {
                        left: Val::Percent(1.0),
                        right: Val::Percent(1.0),
                        top: Val::Percent(1.0),
                        bottom: Val::Percent(1.0),
                    },
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                background_color: FIGHT_UI_BUTTON_COLOR.into(),
                ..Default::default()
            },
            StatusUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "THE BOSSTO",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FIGHT_UI_TEXT_SIZE,
                            color: FIGHT_UI_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "HP: ",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_UI_TEXT_SIZE,
                                color: FIGHT_UI_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                FightEnemyHP,
            ));
        });
}

fn create_buttons_node(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    mut input_mapping: &mut ResMut<InputMapping>,
) {
    input_mapping.keyboard_navigation = true;
    input_mapping.focus_follows_mouse = true;
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(65.0),
                    height: Val::Percent(25.0),
                    row_gap: Val::Px(8.0),
                    column_gap: Val::Px(8.0),
                    padding: UiRect {
                        left: Val::Px(12.0),
                        right: Val::Percent(1.0),
                        top: Val::Percent(1.0),
                        bottom: Val::Percent(1.0),
                    },
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                background_color: FIGHT_UI_NODE_BUTTONS_COLOR.into(),
                ..Default::default()
            },
            PlayerButtonsNode,
        ))
        .with_children(|parent| {
            create_attack_button(parent, &asset_server, input_mapping);
            create_skill_button(parent, &asset_server, &input_mapping);
            create_def_button(parent, &asset_server, &input_mapping);
            create_escape_button(parent, &asset_server, &input_mapping);
            create_status_node(parent, &asset_server, &input_mapping);
        });
}

fn create_attack_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    input_mapping: &ResMut<InputMapping>,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(20.),
                    height: Val::Percent(90.),
                    border: UiRect::all(Val::Px(10.)),
                    //aspect_ratio: Some(1.0),
                    ..Default::default()
                },
                visibility: Visibility::Inherited,
                background_color: FIGHT_UI_BUTTON_COLOR.into(),
                border_color: Color::GOLD.into(),
                ..Default::default()
            },
            FightAttackButton,
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Attack",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FIGHT_UI_TEXT_SIZE,
                            color: FIGHT_UI_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn create_skill_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    mut input_mapping: &ResMut<InputMapping>,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(20.0),
                    height: Val::Percent(90.0),
                    border: UiRect::all(Val::Px(10.)),
                    ..Default::default()
                },
                visibility: Visibility::Inherited,
                background_color: FIGHT_UI_BUTTON_COLOR.into(),
                ..Default::default()
            },
            FightSkillButton,
            Focusable::default(),
            MenuBuilder::Root,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Skill",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FIGHT_UI_TEXT_SIZE,
                            color: FIGHT_UI_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
            create_skill_list_node(parent, &asset_server);
        });
}

fn create_skill_list_node(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    bottom: Val::Percent(-10.),
                    left: Val::Percent(95.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(210.0),
                    height: Val::Percent(200.0),
                    border: UiRect::all(Val::Px(2.)),
                    row_gap: Val::Px(8.0),
                    position_type: PositionType::Absolute,
                    padding: UiRect {
                        left: Val::Percent(2.0),
                        right: Val::Percent(2.0),
                        top: Val::Percent(2.0),
                        bottom: Val::Percent(2.0),
                    },
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                background_color: FIGHT_UI_NODE_BUTTONS_COLOR.into(),
                z_index: ZIndex::Global(900),
                ..Default::default()
            },
            SkillListNode,
            //MenuBuilder::from_named("fight_skill_button")
        ))
        .with_children(|parent| {
            create_node_skill_1(parent, &asset_server);
            create_node_skill_2(parent, &asset_server);
            create_node_skill_3(parent, &asset_server);
            create_node_skill_4(parent, &asset_server);
        });
}

fn create_node_skill_1(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(97.0),
                    height: Val::Percent(25.0),
                    border: UiRect::all(Val::Px(10.)),
                    ..Default::default()
                },
                visibility: Visibility::Inherited,
                background_color: FIGHT_UI_BUTTON_COLOR.into(),
                ..Default::default()
            },
            PlayerSkill {
                name: "FrostBolt".to_string(),
                damage: 20.,
                mana_cost: 20.,
                effect: Some(Debuff::Freezing),
                effect_duration: Some(1.)
            },
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "FrostBolt",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FIGHT_UI_TEXT_SIZE,
                            color: FIGHT_UI_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn create_node_skill_2(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(97.0),
                    height: Val::Percent(25.0),
                    border: UiRect::all(Val::Px(10.)),
                    ..Default::default()
                },
                visibility: Visibility::Inherited,
                background_color: FIGHT_UI_BUTTON_COLOR.into(),
                ..Default::default()
            },
            PlayerSkill {
                name: "FireBolt".to_string(),
                damage: 20.,
                mana_cost: 20.,
                effect: Some(Debuff::Burning),
                effect_duration: Some(2.)
            },
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "FireBolt",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FIGHT_UI_TEXT_SIZE,
                            color: FIGHT_UI_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn create_node_skill_3(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(97.0),
                    height: Val::Percent(25.0),
                    border: UiRect::all(Val::Px(10.)),
                    ..Default::default()
                },
                visibility: Visibility::Inherited,
                background_color: FIGHT_UI_BUTTON_COLOR.into(),
                ..Default::default()
            },
            PlayerSkill {
                name: "ShadowBolt".to_string(),
                damage: 30.,
                mana_cost: 20.,
                effect: None,
                effect_duration: None
            },
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "ShadowBolt",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FIGHT_UI_TEXT_SIZE,
                            color: FIGHT_UI_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn create_node_skill_4(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(97.0),
                    height: Val::Percent(25.0),
                    border: UiRect::all(Val::Px(10.)),
                    ..Default::default()
                },
                visibility: Visibility::Inherited,
                background_color: FIGHT_UI_BUTTON_COLOR.into(),
                ..Default::default()
            },
            PlayerSkill {
                name: "WindFury".to_string(),
                damage: 15.,
                mana_cost: 20.,
                effect: Some(Debuff::Blindness),
                effect_duration: Some(1.)
            },
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "WindFury",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FIGHT_UI_TEXT_SIZE,
                            color: FIGHT_UI_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn create_def_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    mut input_mapping: &ResMut<InputMapping>,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(20.0),
                    height: Val::Percent(90.0),
                    border: UiRect::all(Val::Px(10.)),
                    ..Default::default()
                },
                visibility: Visibility::Inherited,
                background_color: FIGHT_UI_BUTTON_COLOR.into(),
                ..Default::default()
            },
            FightDefButton,
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Defend",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FIGHT_UI_TEXT_SIZE,
                            color: FIGHT_UI_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn create_escape_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    mut input_mapping: &ResMut<InputMapping>,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(20.0),
                    height: Val::Percent(90.0),
                    border: UiRect::all(Val::Px(10.)),
                    ..Default::default()
                },
                visibility: Visibility::Inherited,
                background_color: FIGHT_UI_BUTTON_COLOR.into(),
                ..Default::default()
            },
            FightEscapeButton,
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Escape",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FIGHT_UI_TEXT_SIZE,
                            color: FIGHT_UI_TEXT_COLOR,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn create_status_node(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    mut input_mapping: &ResMut<InputMapping>,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(20.0),
                    height: Val::Percent(90.0),
                    row_gap: Val::Px(8.0),
                    padding: UiRect {
                        left: Val::Percent(1.0),
                        right: Val::Percent(1.0),
                        top: Val::Percent(1.0),
                        bottom: Val::Percent(1.0),
                    },
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                background_color: FIGHT_UI_BUTTON_COLOR.into(),
                ..Default::default()
            },
            FightStatusNode,
            StatusUI
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "HP:",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_UI_TEXT_SIZE,
                                color: FIGHT_UI_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                FightStatusHP,
            ));

            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "MP: ",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_UI_TEXT_SIZE,
                                color: FIGHT_UI_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                FightStatusMP,
            ));
        });
}
