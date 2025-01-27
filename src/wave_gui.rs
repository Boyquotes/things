use bevy::{
    color::palettes::css::{BLUE, GRAY},
    prelude::*,
};
use bevy_simple_text_input::{
    TextInputBundle, TextInputInactive, TextInputSettings, TextInputSubmitEvent, TextInputValue,
};

#[derive(Resource)]
pub struct GuiInputs {
    pub amplitude: String,
    pub wave_length: String,
    pub frequency: String,
    pub k_coefficient: String,
    pub angular_frequency_coefficient: String,
    pub phase: String,
}

#[derive(Event, Default, Debug)]
pub struct GuiInputsEvent {
    pub amplitude: String,
    pub wave_length: String,
    pub frequency: String,
    pub k_coefficient: String,
    pub angular_frequency_coefficient: String,
    pub phase: String,
}

#[derive(Resource)]
pub struct GuiInputEntities {
    pub amplitude: Entity,
    pub wave_length: Entity,
    pub frequency: Entity,
    pub k_coefficient: Entity,
    pub angular_frequency_coefficient: Entity,
    pub phase: Entity,
}

/// marker component for amplitude text input
/// needs to be public to add the component in main, maybe I restructure this later
#[derive(Component, Default)]
pub struct AmplitudeInputMarker;
#[derive(Component, Default)]
pub struct WaveLengthInputMarker;
#[derive(Component, Default)]
pub struct FrequencyInputMarker;
#[derive(Component, Default)]
pub struct KCoefficientMarker;
#[derive(Component, Default)]
pub struct AngularCoefficientMarker;
#[derive(Component, Default)]
pub struct PhaseMarker;

pub fn setup_wave_gui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    form_state: Res<GuiInputs>,
) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let root = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            width: Val::Px(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    });

    let root_id = root.id();

    let amplitude_input = generate_input_box(
        &font,
        root_id,
        &mut commands,
        "Amplitude",
        AmplitudeInputMarker,
        form_state.amplitude.clone(),
    );
    let wave_length_input = generate_input_box(
        &font,
        root_id,
        &mut commands,
        "Wave length",
        WaveLengthInputMarker,
        form_state.wave_length.clone(),
    );
    let frequency_input = generate_input_box(
        &font,
        root_id,
        &mut commands,
        "Frequency",
        FrequencyInputMarker,
        form_state.frequency.clone(),
    );
    let k_coefficient_input = generate_input_box(
        &font,
        root_id,
        &mut commands,
        "K coefficient",
        KCoefficientMarker,
        form_state.k_coefficient.clone(),
    );
    let angular_frequency_coefficient_input = generate_input_box(
        &font,
        root_id,
        &mut commands,
        "Angular frequency coefficient",
        AngularCoefficientMarker,
        form_state.angular_frequency_coefficient.clone(),
    );
    let phase_input = generate_input_box(
        &font,
        root_id,
        &mut commands,
        "Phase",
        PhaseMarker,
        form_state.phase.clone(),
    );

    commands.insert_resource(GuiInputEntities {
        amplitude: amplitude_input,
        wave_length: wave_length_input,
        frequency: frequency_input,
        k_coefficient: k_coefficient_input,
        angular_frequency_coefficient: angular_frequency_coefficient_input,
        phase: phase_input,
    });
}

fn generate_input_box<T>(
    font: &Handle<Font>,
    root_id: Entity,
    commands: &mut Commands,
    label: &str,
    marker: T,
    value: String,
) -> Entity
where
    T: Component,
{
    let label = generate_input_label(font, label);
    let wrapper = generate_input_wrapper();
    let text_input_bundle = generate_input(value);

    let spawned_label = commands.spawn(label).id();
    commands.entity(root_id).push_children(&[spawned_label]);

    let spawned_wrapper = commands.spawn(wrapper).id();
    commands.entity(root_id).push_children(&[spawned_wrapper]);

    let spawned_text_input_bundle = commands.spawn((marker, text_input_bundle)).id();
    commands
        .entity(spawned_wrapper)
        .push_children(&[spawned_text_input_bundle]);

    spawned_text_input_bundle
}

fn generate_input_label(font: &Handle<Font>, label: &str) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Auto,
            ..default()
        },
        text: Text::from_section(
            label.to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    }
}

fn generate_input_wrapper() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            margin: UiRect {
                bottom: Val::Px(20.0),
                ..default()
            },
            ..default()
        },
        ..default()
    }
}

fn generate_input(value: String) -> (NodeBundle, TextInputBundle) {
    let input = TextStyle {
        font_size: 14.,
        color: Color::WHITE,
        ..default()
    };

    (
        NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            border_color: GRAY.into(),
            background_color: GRAY.into(),
            ..default()
        },
        TextInputBundle {
            settings: TextInputSettings {
                retain_on_submit: true,
                ..default()
            },
            inactive: TextInputInactive(true),
            value: TextInputValue(value),
            ..default()
        }
        .with_text_style(input),
    )
}

pub fn text_listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut inputs: ResMut<GuiInputs>,
    input_entities: Res<GuiInputEntities>,
) {
    for event in events.read() {
        if event.entity == input_entities.amplitude {
            info!("submitted amplitude: {}", event.value);
            inputs.amplitude = event.value.clone();
        } else if event.entity == input_entities.wave_length {
            println!("submitted wave length: {:?}", event.entity);
            inputs.wave_length = event.value.clone();
        } else if event.entity == input_entities.frequency {
            println!("submitted frequency: {:?}", event.entity);
            inputs.frequency = event.value.clone();
        } else if event.entity == input_entities.k_coefficient {
            println!("submitted k coefficient: {:?}", event.entity);
            inputs.k_coefficient = event.value.clone();
        } else if event.entity == input_entities.angular_frequency_coefficient {
            println!(
                "submitted angular frequency coefficient: {:?}",
                event.entity
            );
            inputs.angular_frequency_coefficient = event.value.clone();
        } else if event.entity == input_entities.phase {
            println!("submitted phase: {:?}", event.entity);
            inputs.phase = event.value.clone();
        } else {
            println!("unknown entity: {:?}", event.entity);
        }
    }
}

pub fn form_state_notifier_system(
    form_state: Res<GuiInputs>,
    mut my_events: EventWriter<GuiInputsEvent>,
) {
    my_events.send(GuiInputsEvent {
        amplitude: form_state.amplitude.clone(),
        wave_length: form_state.wave_length.clone(),
        frequency: form_state.frequency.clone(),
        k_coefficient: form_state.k_coefficient.clone(),
        angular_frequency_coefficient: form_state.angular_frequency_coefficient.clone(),
        phase: form_state.phase.clone(),
    });
}

// TODO error handling (show on ui)
#[allow(clippy::too_many_arguments)]
pub fn listen_gui_inputs(
    mut events: EventReader<GuiInputsEvent>,
    mut commands: Commands,
    amplitude_query: Query<Entity, With<Amplitude>>,
    wave_length_query: Query<Entity, With<WaveLength>>,
    frequency_query: Query<Entity, With<Frequency>>,
    k_coefficient_query: Query<Entity, With<KCoefficient>>,
    angular_frequency_coefficient_query: Query<Entity, With<AngularFrequencyCoefficient>>,
    phase_query: Query<Entity, With<Phase>>,
) {
    for input in events.read() {
        // println!("got events in wave.rs: {:?}", input);
        match parse_float(&input.amplitude) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &amplitude_query);
                commands.spawn(Amplitude(f));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.wave_length) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &wave_length_query);
                commands.spawn(WaveLength(f));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.frequency) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &frequency_query);
                commands.spawn(Frequency(f));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.k_coefficient) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &k_coefficient_query);
                commands.spawn(KCoefficient(f));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.angular_frequency_coefficient) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &angular_frequency_coefficient_query);
                commands.spawn(AngularFrequencyCoefficient(f));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.phase) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &phase_query);
                commands.spawn(Phase(f));
            }
            Err(err) => println!("error: {}", err),
        }
    }
}

fn parse_float(str: &str) -> Result<f32, String> {
    let f = str.parse::<f32>();
    match f {
        Ok(f) => Ok(f),
        Err(e) => Err(format!("Failed to parse float: {}", e)),
    }
}

fn despawn_all_entities<T>(commands: &mut Commands, query: &Query<Entity, With<T>>)
where
    T: Component,
{
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = BLUE.into();
                } else {
                    inactive.0 = true;
                    *border_color = GRAY.into();
                }
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct Amplitude(pub f32);

#[derive(Component, Debug)]
pub struct WaveLength(pub f32);

#[derive(Component, Debug)]
pub struct Frequency(pub f32);

#[derive(Component, Debug)]
pub struct KCoefficient(pub f32);

#[derive(Component, Debug)]
pub struct AngularFrequencyCoefficient(pub f32);

#[derive(Component, Debug)]
pub struct Phase(pub f32);
