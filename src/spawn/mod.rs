fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    let size = Vec2::new(20.0, 20.0);

    let mut velocity = Vec2::new(200.0, 200.0);
    let speed = velocity.length();
    velocity = velocity.normalize() * speed;

    let texture = asset_server.load("textures/ball.png");

    commands.spawn((
        Sprite {
            image: texture,
            custom_size: Some(size),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, -220.0, 0.0)),
        GlobalTransform::default(),
        Ball { velocity },
        Collider { size },
        GameEntity,
    ));
}