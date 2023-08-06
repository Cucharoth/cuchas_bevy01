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
        .spawn((
            NodeBundle {
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
            FightNodeRoot,
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
            ImageBundle {
                image: UiImage {
                    texture: asset_server.load("UI/tile01.png"),
                    ..Default::default()
                },
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
                background_color: FIGHT_UI_NODE_BUTTONS_COLOR.into(),
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
            ImageBundle {
                image: UiImage {
                    texture: asset_server.load("UI/tile01.png"),
                    ..Default::default()
                },
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
                z_index: ZIndex::Global(400),
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
                z_index: ZIndex::Global(500),
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
                z_index: ZIndex::Global(500),
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
            ImageBundle {
                image: UiImage {
                    texture: asset_server.load("UI/tile01.png"),
                    ..Default::default()
                },
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
                z_index: ZIndex::Global(550),
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
                z_index: ZIndex::Global(551),
                ..Default::default()
            },
            PlayerSkill {
                name: "FrostBolt".to_string(),
                damage: 20.,
                mana_cost: 20.,
                effect: Some(Debuff::Freezing),
                effect_duration: Some(1.),
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
                z_index: ZIndex::Global(551),
                ..Default::default()
            },
            PlayerSkill {
                name: "FireBolt".to_string(),
                damage: 20.,
                mana_cost: 20.,
                effect: Some(Debuff::Burning),
                effect_duration: Some(2.),
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
                z_index: ZIndex::Global(551),
                ..Default::default()
            },
            PlayerSkill {
                name: "ShadowBolt".to_string(),
                damage: 30.,
                mana_cost: 20.,
                effect: None,
                effect_duration: None,
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
                z_index: ZIndex::Global(551),
                ..Default::default()
            },
            PlayerSkill {
                name: "WindFury".to_string(),
                damage: 15.,
                mana_cost: 20.,
                effect: Some(Debuff::Blindness),
                effect_duration: Some(1.),
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
                z_index: ZIndex::Global(500),
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
                z_index: ZIndex::Global(500),
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
                z_index: ZIndex::Global(500),
                ..Default::default()
            },
            FightStatusNode,
            StatusUI,
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

pub fn combat_log_root(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            (NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Row,
                    //align_items: AlignItems::End,
                    //align_self: AlignSelf::Center,
                    //justify_self: JustifySelf::Center,
                    justify_content: JustifyContent::End,
                    //left: Val::Percent(50.),
                    //bottom: Val::Percent(70.),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    //row_gap: Val::Px(8.0),
                    /*padding: UiRect {
                        left: Val::Percent(1.0),
                        right: Val::Percent(1.0),
                        top: Val::Percent(1.0),
                        bottom: Val::Percent(1.0),
                    }*/
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                background_color: FIGHT_UI_BUTTON_COLOR.into(),
                ..Default::default()
            },
            CombatLogRoot
        ),
        )
        .with_children(|parent| {
            create_combat_log_buttons(parent, &asset_server);
        });
}

fn create_combat_log_buttons(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                //align_self: AlignSelf::Center,
                //justify_self: JustifySelf::Center,
                justify_content: JustifyContent::Center,
                //left: Val::Percent(50.),
                //bottom: Val::Percent(70.),
                width: Val::Percent(27.0),
                height: Val::Percent(30.0),
                //row_gap: Val::Px(2.0),
                /*padding: UiRect {
                    left: Val::Percent(1.0),
                    right: Val::Percent(1.0),
                    top: Val::Percent(1.0),
                    bottom: Val::Percent(1.0),
                }*/
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            background_color: Color::hsla(338.0, 0.8, 0.100, 0.8).into(),
            z_index: ZIndex::Global(1),
            ..Default::default()
        },
        CombatLogButtons
    ),
    )
    .with_children(
        |parent| {
            create_node_combat_log_1(parent, &asset_server);
            create_node_combat_log_2(parent, &asset_server);
            create_node_combat_log_3(parent, &asset_server);
            create_node_combat_log_4(parent, &asset_server);
            create_node_combat_log_5(parent, &asset_server);
            create_node_combat_log_6(parent, &asset_server);
            create_node_combat_log_7(parent, &asset_server);
            create_node_combat_log_8(parent, &asset_server);
            create_node_combat_log_9(parent, &asset_server);
            create_node_combat_log_10(parent, &asset_server);
        }
    );
}

fn create_node_combat_log_1(
    parent: &mut ChildBuilder, asset_server: &Res<AssetServer>
) {
    parent.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(95.0),
                height: Val::Percent(9.3),
                ..Default::default()
            },
            visibility: Visibility::Inherited,
            background_color: FIGHT_COMBAT_LOG_NODE_COLOR.into(),
            ..Default::default()
        }),
    )
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_COMBAT_LOG_TEXT_SIZE,
                                color: FIGHT_COMBAT_LOG_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..Default::default()
                    },
                    z_index: ZIndex::Global(100),
                    ..Default::default()
                },
                CombatLogText1,
                LogText
            ));
        }
    );
}

fn create_node_combat_log_2(
    parent: &mut ChildBuilder, asset_server: &Res<AssetServer>
) {
    parent.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(95.0),
                height: Val::Percent(9.3),
                ..Default::default()
            },
            visibility: Visibility::Inherited,
            background_color: FIGHT_COMBAT_LOG_NODE_COLOR.into(),
            ..Default::default()
        }),
    )
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_COMBAT_LOG_TEXT_SIZE,
                                color: FIGHT_COMBAT_LOG_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CombatLogText2,
                LogText
            ));
        }
    );
}

fn create_node_combat_log_3(
    parent: &mut ChildBuilder, asset_server: &Res<AssetServer>
) {
    parent.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(95.0),
                height: Val::Percent(9.3),
                ..Default::default()
            },
            visibility: Visibility::Inherited,
            background_color: FIGHT_COMBAT_LOG_NODE_COLOR.into(),
            ..Default::default()
        }),
    )
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_COMBAT_LOG_TEXT_SIZE,
                                color: FIGHT_COMBAT_LOG_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CombatLogText3,
                LogText
            ));
        }
    );
}

fn create_node_combat_log_4(
    parent: &mut ChildBuilder, asset_server: &Res<AssetServer>
) {
    parent.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(95.0),
                height: Val::Percent(9.3),
                ..Default::default()
            },
            visibility: Visibility::Inherited,
            background_color: FIGHT_COMBAT_LOG_NODE_COLOR.into(),
            ..Default::default()
        }),
    )
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_COMBAT_LOG_TEXT_SIZE,
                                color: FIGHT_COMBAT_LOG_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CombatLogText4,
                LogText
            ));
        }
    );
}

fn create_node_combat_log_5(
    parent: &mut ChildBuilder, asset_server: &Res<AssetServer>
) {
    parent.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(95.0),
                height: Val::Percent(9.3),
                ..Default::default()
            },
            visibility: Visibility::Inherited,
            background_color: FIGHT_COMBAT_LOG_NODE_COLOR.into(),
            ..Default::default()
        }),
    )
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_COMBAT_LOG_TEXT_SIZE,
                                color: FIGHT_COMBAT_LOG_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CombatLogText5,
                LogText
            ));
        }
    );
}

fn create_node_combat_log_6(
    parent: &mut ChildBuilder, asset_server: &Res<AssetServer>
) {
    parent.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(95.0),
                height: Val::Percent(9.3),
                ..Default::default()
            },
            visibility: Visibility::Inherited,
            background_color: FIGHT_COMBAT_LOG_NODE_COLOR.into(),
            ..Default::default()
        }),
    )
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_COMBAT_LOG_TEXT_SIZE,
                                color: FIGHT_COMBAT_LOG_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CombatLogText6,
                LogText
            ));
        }
    );
}

fn create_node_combat_log_7(
    parent: &mut ChildBuilder, asset_server: &Res<AssetServer>
) {
    parent.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(95.0),
                height: Val::Percent(9.3),
                ..Default::default()
            },
            visibility: Visibility::Inherited,
            background_color: FIGHT_COMBAT_LOG_NODE_COLOR.into(),
            ..Default::default()
        }),
    )
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_COMBAT_LOG_TEXT_SIZE,
                                color: FIGHT_COMBAT_LOG_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CombatLogText7,
                LogText
            ));
        }
    );
}

fn create_node_combat_log_8(
    parent: &mut ChildBuilder, asset_server: &Res<AssetServer>
) {
    parent.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(95.0),
                height: Val::Percent(9.3),
                ..Default::default()
            },
            visibility: Visibility::Inherited,
            background_color: FIGHT_COMBAT_LOG_NODE_COLOR.into(),
            ..Default::default()
        }),
    )
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_COMBAT_LOG_TEXT_SIZE,
                                color: FIGHT_COMBAT_LOG_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CombatLogText8,
                LogText
            ));
        }
    );
}

fn create_node_combat_log_9(
    parent: &mut ChildBuilder, asset_server: &Res<AssetServer>
) {
    parent.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(95.0),
                height: Val::Percent(9.3),
                ..Default::default()
            },
            visibility: Visibility::Inherited,
            background_color: FIGHT_COMBAT_LOG_NODE_COLOR.into(),
            ..Default::default()
        }),
    )
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_COMBAT_LOG_TEXT_SIZE,
                                color: FIGHT_COMBAT_LOG_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CombatLogText9,
                LogText
            ));
        }
    );
}

fn create_node_combat_log_10(
    parent: &mut ChildBuilder, asset_server: &Res<AssetServer>
) {
    parent.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(95.0),
                height: Val::Percent(9.3),
                ..Default::default()
            },
            visibility: Visibility::Inherited,
            background_color: FIGHT_COMBAT_LOG_NODE_COLOR.into(),
            ..Default::default()
        }),
    )
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: FIGHT_COMBAT_LOG_TEXT_SIZE,
                                color: FIGHT_COMBAT_LOG_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Left,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CombatLogText10,
                LogText
            ));
        }
    );
}