use bevy::prelude::*;
use bevy_ui_navigation::prelude::Focusable;
use crate::systems::ui::pause_menu::systems::components::*;
use crate::systems::ui::pause_menu::systems::style::*;


pub fn pause_menu_root(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    //align_self: AlignSelf::Center,
                    //justify_self: JustifySelf::Center,
                    justify_content: JustifyContent::Center,
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
                background_color: PAUSE_MENU_BACKGROUND.into(),
                ..Default::default()
            },
            PauseRootNode,
        ))
        .with_children(|parent| {
            create_menu_node(parent, &asset_server);
        });
}

fn create_menu_node(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(
            (NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    //left: Val::Percent(50.),
                    //bottom: Val::Percent(30.),
                    row_gap: Val::Percent(15.0),
                    column_gap: Val::Percent(0.5),
                    width: Val::Percent(20.0),
                    height: Val::Percent(40.0),
                    ..Default::default()
                },
                visibility: Visibility::Visible,
                background_color: PAUSE_MENU_BACKGROUND.into(),
                z_index: ZIndex::Global(999),
                ..Default::default()
            },
            PauseMenuNode,
            ))
        .with_children(|parent| {
            create_pause_text_node(parent, &asset_server);
            create_resume_button(parent, &asset_server);
            create_main_menu_button(parent, &asset_server);
        });
}

fn create_pause_text_node(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
        parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Start,
                    //align_self: AlignSelf::Center,
                    //justify_self: JustifySelf::Center,
                    justify_content: JustifyContent::Center,
                    //left: Val::Percent(50.),
                    bottom: Val::Percent(5.),
                    width: Val::Percent(90.0),
                    height: Val::Percent(20.0),
                    //row_gap: Val::Px(8.0),
                    /*padding: UiRect {
                        left: Val::Percent(1.0),
                        right: Val::Percent(1.0),
                        top: Val::Percent(1.0),
                        bottom: Val::Percent(1.0),
                    }*/
                    ..Default::default()
                },
                visibility: Visibility::Inherited,
                background_color: PAUSE_MENU_BUTTON_BACKGROUND.into(),
                ..Default::default()
            },
            PauseRootNode,
        ))
        .with_children(|parent| {
        parent.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "PAUSE",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.,
                        color: PAUSE_MENU_PAUSE_COLOR,
                    },
                )],
                alignment: TextAlignment::Center,
                ..Default::default()
            },
            ..Default::default()
        },
        PauseMenuText,
));});

}

fn create_resume_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(90.0),
                    height: Val::Percent(10.0),
                    ..Default::default()
                },
                visibility: Visibility::Inherited,
                background_color: PAUSE_MENU_BUTTON_BACKGROUND.into(),
                ..Default::default()
            },
            PauseMenuResumeNode,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Resume",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: PAUSE_MENU_TEXT_SIZE,
                                color: PAUSE_MENU_BUTTON_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                PauseMenuResumeText,
                PauseText,
                Focusable::default()
            ));
        });
}

fn create_main_menu_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(90.0),
                    height: Val::Percent(10.0),
                    ..Default::default()
                },
                visibility: Visibility::Inherited,
                background_color: PAUSE_MENU_BUTTON_BACKGROUND.into(),
                ..Default::default()
            },
            PauseMenuMainMenuNode,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Main Menu",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: PAUSE_MENU_TEXT_SIZE,
                                color: PAUSE_MENU_BUTTON_TEXT_COLOR,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                PauseMenuMainMenuText,
                PauseText,
                Focusable::default()
            ));
        });
}
