# Structure — Breakout LB426

## Общая структура проекта

```
Breakout_LB426/
├── Cargo.toml              # Конфигурация проекта Rust
├── Cargo.lock              # Заблокированные версии зависимостей
├── config.json             # Сохраняемые настройки игры
├── Listing.md              # Листинг всего исходного кода
├── structure.md            # Этот файл (структура проекта)
├── project.md              # Описание проекта
├── .vscode/
│   └── launch.json         # Конфигурация отладки VS Code
├── assets/
│   ├── fonts/
│   │   └── FiraSans-Bold.ttf    # Шрифт для UI
│   └── sounds/                  # Звуковые эффекты (bounce, break)
└── src/
    ├── main.rs                  # Точка входа
    ├── app/                     # Модуль приложения
    │   ├── mod.rs
    │   ├── plugins.rs           # AppPlugins — главный плагин
    │   └── states.rs            # GameState — состояния игры
    ├── core/                    # Базовые системы
    │   ├── mod.rs
    │   ├── config.rs            # GameConfig + ConfigPlugin (JSON)
    │   ├── camera.rs            # setup_camera
    │   ├── assets.rs            # заготовка
    │   └── time.rs              # заготовка
    ├── gameplay/                # Игровая логика
    │   ├── mod.rs
    │   ├── plugin.rs            # GameplayPlugin
    │   ├── components/
    │   │   ├── mod.rs
    │   │   ├── paddle.rs        # Paddle
    │   │   ├── ball.rs          # Ball
    │   │   ├── velocity.rs      # Velocity
    │   │   ├── collider.rs      # Collider
    │   │   └── brick.rs         # Brick, BrickType
    │   ├── systems/
    │   │   ├── mod.rs
    │   │   ├── inputs.rs        # paddle_input, game_pause
    │   │   ├── movement.rs      # ball_movement, paddle_movement
    │   │   └── collision/
    │   │       ├── mod.rs
    │   │       ├── wall.rs      # ball_wall_collision
    │   │       ├── paddle.rs    # ball_paddle_collision
    │   │       ├── bricks.rs    # ball_brick_collision
    │   │       └── death.rs     # ball_death
    │   ├── resources/
    │   │   ├── mod.rs
    │   │   ├── score.rs         # Score(u32)
    │   │   └── lives.rs         # Lives(u32)
    │   └── spawn/
    │       ├── mod.rs           # GameEntity, spawn_game, cleanup_game
    │       └── level.rs         # Block, spawn_block, setup_level
    ├── ui/
    │   ├── mod.rs
    │   ├── plugin.rs            # UiPlugin — все экраны
    │   ├── screens/
    │   │   ├── mod.rs
    │   │   ├── main_menu.rs     # MainMenu экран
    │   │   ├── pause.rs         # Pause экран
    │   │   ├── settings.rs      # Settings экран
    │   │   └── game_over.rs     # GameOver экран
    │   ├── components/
    │   │   ├── mod.rs
    │   │   └── ui_root.rs       # заготовка
    │   └── systems/
    │       ├── mod.rs
    │       └── navigation.rs    # заготовка
    ├── components/              # устаревший модуль (re-export)
    │   └── mod.rs
    ├── systems/                 # устаревший модуль (заготовки)
    │   ├── mod.rs
    │   ├── config.rs
    │   └── collision/
    ├── spawn/                   # устаревший модуль (заготовки)
    └── utils/
        ├── mod.rs
        └── math.rs              # заготовка
```

---

## Модульная структура

### `src/app/`

| Файл | Описание |
|------|----------|
| `plugins.rs` | `AppPlugins` — регистрирует `DefaultPlugins`, камеру (`Startup`), и подплагины |
| `states.rs` | `GameState` — машина состояний игры |

**Состояния (`GameState`):**

```
MainMenu (по умолчанию)
    ↓ Enter
  Playing ──ESC──► Paused ──ESC──► Playing
    │                │
  0 жизней        Enter
    ↓                ↓
  GameOver        MainMenu
    │
  Space
    ↓
  Playing
```

---

### `src/core/`

| Файл | Описание |
|------|----------|
| `config.rs` | `GameConfig` (window_width, window_height, music_volume, sfx_volume) + `ConfigPlugin` — загрузка/сохранение `config.json` |
| `camera.rs` | `setup_camera` — создание `Camera2d`, `camera_scaling` — масштабирование камеры при изменении размера окна |

---

### `src/gameplay/`

#### Компоненты (`components/`)

| Файл | Компонент | Поля |
|------|-----------|------|
| `paddle.rs` | `Paddle` | `speed: f32`, `direction: f32` |
| `ball.rs` | `Ball` | `speed: f32`, `velocity: Vec2` |
| `collider.rs` | `Collider` | `size: Vec2` |
| `brick.rs` | `Brick`, `BrickType` | `brick_type`, `health: u8`, `score: u32` |
| `velocity.rs` | `Velocity(Vec2)` | общий компонент (не используется) |

#### Ресурсы (`resources/`)

| Файл | Ресурс | Начальное значение |
|------|--------|--------------------|
| `score.rs` | `Score(pub u32)` | 0 |
| `lives.rs` | `Lives(pub u32)` | 3 |

`resources/mod.rs` реэкспортирует `Lives` и `Score` напрямую (`pub use`).

#### Системы (`systems/`)

`systems/mod.rs` реэкспортирует все функции через `pub use`, чтобы `plugin.rs` мог использовать glob-импорт.

| Система | Файл | Описание |
|---------|------|----------|
| `paddle_input` | `inputs.rs` | A/D/←/→ → устанавливает `paddle.direction` |
| `game_pause` | `inputs.rs` | ESC → `GameState::Paused` |
| `ball_movement` | `movement.rs` | `translation += velocity * delta_secs` |
| `paddle_movement` | `movement.rs` | движение + зажим по границам экрана |
| `ball_wall_collision` | `collision/wall.rs` | отскок от левой/правой/верхней стены |
| `ball_paddle_collision` | `collision/paddle.rs` | отскок с угловым смещением по X |
| `ball_brick_collision` | `collision/bricks.rs` | AABB + определение оси отскока по перекрытию |
| `ball_death` | `collision/death.rs` | мяч ниже экрана → `lives -= 1`, сброс, или `GameOver` |

#### Спавн (`spawn/`)

| Функция | Описание |
|---------|----------|
| `spawn_game` | Вызывается `OnEnter(Playing)`. Проверяет наличие `Ball` — если уже есть (возврат из паузы), ничего не делает |
| `cleanup_game` | Вызывается `OnEnter(MainMenu)`. Уничтожает все сущности с `GameEntity` |
| `setup_level` | Создаёт сетку блоков 5×10. Чётные ряды — `Normal` (1 хит, 100 очков), нечётные — `Strong` (2 хита, 200 очков) |

**Маркер:** `GameEntity` — компонент-тег для всех игровых объектов (платформа, мяч, блоки). Используется для массового удаления.

---

### `src/ui/`

`ui/plugin.rs` (`UiPlugin`) регистрирует все экраны через `OnEnter`/`OnExit`/`Update`.

#### Экраны (`screens/`)

| Экран | Состояние | Системы |
|-------|-----------|---------|
| `main_menu.rs` | `MainMenu` | `setup_main_menu`, `cleanup_main_menu`, `main_menu_input`, `update_menu_visuals` |
| `pause.rs` | `Paused` | `setup_pause_ui`, `cleanup_pause_ui`, `pause_input` |
| `settings.rs` | `Settings` | `setup_settings_ui`, `cleanup_settings_ui`, `settings_input` |
| `game_over.rs` | `GameOver` | `setup_game_over`, `cleanup_game_over`, `restart_game` |

`screens/mod.rs` реэкспортирует все функции через `pub use`.

---

## Игровое поле

| Параметр | Значение |
|----------|----------|
| Размер окна | 800×600 |
| Виртуальный размер игрового мира | 800×600 |
| Платформа | 120×20, Y = −250 |
| Мяч | 20×20, старт Y = −220 |
| Блоки | 50×20, 5 рядов × 10 столбцов, старт Y = 250 |
| Расстояние между блоками | 5 пикселей |
| Скорость платформы | 500 пикс/сек |
| Начальная скорость мяча | Vec2(200, 200) нормализованная |
| Половина ширины экрана (для коллизий) | 400 пикселей |

---

## Управление

### Главное меню
| Клавиша | Действие |
|---------|----------|
| `↑` / `↓` | Выбор пункта |
| `Enter` | Подтвердить |

### Игра
| Клавиша | Действие |
|---------|----------|
| `A` / `←` | Движение влево |
| `D` / `→` | Движение вправо |
| `ESC` | Пауза |

### Пауза
| Клавиша | Действие |
|---------|----------|
| `ESC` | Продолжить |
| `Enter` | В главное меню |

### Настройки
| Клавиша | Действие |
|---------|----------|
| `↑` / `↓` | Выбор пункта |
| `←` / `→` | Изменение значения |
| `Enter` | Сохранить и выйти |
| `ESC` | Выйти без сохранения |

### Game Over
| Клавиша | Действие |
|---------|----------|
| `Space` | Начать заново |

---

## Архитектурные паттерны

### Plugin Architecture

```
AppPlugins
├── DefaultPlugins (Bevy с окном 800×600, заголовок "Breakout")
├── setup_camera (Startup)
├── camera_scaling (Update)
├── ConfigPlugin
├── GameplayPlugin
└── UiPlugin
```

### Коллизии (AABB)

Все коллизии — самописные AABB без Rapier. Вспомогательная функция `collide(a_pos, a_size, b_pos, b_size) -> bool` дублируется в `collision/paddle.rs` и `collision/bricks.rs`. В `bricks.rs` дополнительно вычисляется перекрытие по осям X/Y для определения направления отскока.

---

## Зависимости

| Зависимость | Версия | Назначение |
|-------------|--------|------------|
| `bevy` | 0.18.1 | Игровой движок (ECS, рендер, ввод) |
| `serde` | 1.0 | Сериализация |
| `serde_json` | 1.0 | JSON для `config.json` |

---

**Дата обновления:** 30 марта 2026 г. (Окно 800×600, camera_scaling, нормализованная скорость мяча)
