use crate::{History, Model, Msg, Stage, TOKENS, Player, Token, Camera, Drag};
use seed::prelude::*;

const ENTER_KEY: u32 = 13;

/// Main view
pub fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        h1!["PlayWithMe Rust/Seed edition!"],
        hr![],
        match model.stage {
            Stage::Lobby => lobby(model),
            Stage::Loading => div!["Loading..."],
            Stage::Gameplay => gameplay(model),
        },
    ]
}

fn lobby(model: &Model) -> Node<Msg> {
    div![
        id!("lobby"),
        label![
            "Name",
            input![
                attrs! {At::Value => model.player.name, At::AutoFocus => true},
                input_ev(Ev::Input, Msg::NameChange),
                keyboard_ev(Ev::KeyDown, submit),
            ]
        ],
        br![],
        label![
            "Session",
            input![
                attrs! {At::Value => model.session},
                input_ev(Ev::Input, Msg::SessionChange),
                keyboard_ev(Ev::KeyDown, submit),
            ]
        ],
        br![],
        button!["Create/Connect", simple_ev(Ev::Click, Msg::Connect)],
    ]
}

fn gameplay(model: &Model) -> Node<Msg> {
    div![
        div![
            id!("gameplay-header"),
            div![
                id!("controls"),
                div![
                    id!["tokens"],
                    label!["Token"],
                    TOKENS
                        .iter()
                        .map(|token| {
                            button![
                                class![if token == &model.player.token.code {
                                    "selected"
                                } else {
                                    ""
                                }],
                                token,
                                simple_ev(Ev::Click, Msg::TokenChange(token.to_string()))
                            ]
                        })
                        .collect::<Vec<_>>()
                ],
                label![
                    id!["color"],
                    "Color",
                    input![
                        attrs!{At::Type => "color"},
                        input_ev(Ev::Change, Msg::TokenColorChange)
                    ],
                ],
                button!["Refresh", simple_ev(Ev::Click, Msg::CleanHistory)],
            ],
            label![
                id!{"session"},
                "Session",
                input![attrs! {At::Value => model.session, At::ReadOnly => true }]
            ],
        ],
        hr![],
        div![
            id!("gameplay-area"),
            player_list(&model.players, &model.player),
            div![
                id!("grid-container"),

                simple_ev(Ev::PointerUp, Msg::Drag(Drag::Stop)),
                simple_ev(Ev::PointerDown, Msg::Drag(Drag::Start)),
                pointer_ev(Ev::PointerMove, |ev| {
                    ev.prevent_default();
                    Msg::Drag(Drag::Move{x: ev.movement_x(), y: ev.movement_y()})
                }),

                draw_grid(&model.camera, model.size, &model.history),
            ]
        ]
    ]
}

fn submit(ev: web_sys::KeyboardEvent) -> Msg {
    if ev.key_code() == ENTER_KEY {
        Msg::Connect
    } else {
        Msg::Nope
    }
}

fn player_list(players: &[Player], player: &Player) -> Node<Msg> {
    ul![
        id!("players"),
        li![class!["list-header"], "Players:"],
        players
            .iter()
            .map(|p| li![class![if p.id == player.id { "is-player" } else { "" }], p.name])
            .collect::<Vec<_>>(),
    ]
}

fn draw_grid(camera: &Camera, size: u32, history: &History) -> Node<Msg> {
    div![
        class!["grid"],
        style!{
            "left" => unit!(camera.x, px),
            "top" => unit!(camera.y, px),
        },
        (0..size)
            .map(|y| draw_row(size, y, history))
            .collect::<Vec<_>>()
    ]
}

fn draw_row(size: u32, y: u32, history: &History) -> Node<Msg> {
    div![
        class!["row"],
        (0..size)
            .map(|x| {
                let token = history
                    .iter()
                    .find(|cell| cell.coord.y == y && cell.coord.x == x)
                    .map(|cell| cell.value.clone())
                    .unwrap_or_else(|| Token{code: " ".into(), color: "".into()});
                div![
                    id!(&format!("cell-{}-{}", x, y)),
                    class!["cell"],
                    if token.color != "" {
                        style!{"color" => token.color}
                    } else {
                        style!{}
                    },
                    simple_ev(Ev::Click, Msg::Move { x, y }),
                    token.code,
                ]
            })
            .collect::<Vec<_>>()
    ]
}
