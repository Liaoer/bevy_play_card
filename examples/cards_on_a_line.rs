use bevy_card::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyCardPlugin::default()))
        .add_systems(Startup, (setup, spawn_cards_on_a_line).chain())
        .run();
}

fn spawn_cards_on_a_line(
    mut card_line_request_writer: EventWriter<CardLineRequest>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let cards_count = 12;
    let line = commands
        .spawn(CardLineBundle::from_card_line(CardLine {
            max_cards: cards_count,
            card_origin_gap: 100.0,
            ..default()
        }))
        .id();
    for _ in 0..cards_count {
        let card_entity = commands
            .spawn((
                Sprite {
                    image: asset_server.load("PlaceholderCard.png"),
                    ..default()
                },
                CardBundle::new(Transform::from_scale(Vec3::splat(0.4))),
            ))
            .id();
        card_line_request_writer.send(CardLineRequest {
            line,
            request_type: CardLineRequestType::AddToCardLine { card_entity },
        });
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
