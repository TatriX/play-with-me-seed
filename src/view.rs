use crate::{History, Model, Msg, Stage, TOKENS, Player, Token};
use seed::prelude::*;

const ENTER_KEY: u32 = 13;

/// Main view
pub fn view(model: &Model) -> Vec<El<Msg>> {
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

fn lobby(model: &Model) -> El<Msg> {
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

fn gameplay(model: &Model) -> El<Msg> {
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
                draw_grid(model.size, &model.history),
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

fn player_list(players: &[Player], player: &Player) -> El<Msg> {
    ul![
        id!("players"),
        li![class!["list-header"], "Players:"],
        players
            .iter()
            .map(|p| li![class![if p.id == player.id { "is-player" } else { "" }], p.name])
            .collect::<Vec<_>>(),
    ]
}

fn draw_grid(size: u32, history: &History) -> El<Msg> {
    div![
        class!["grid"],
        (0..size)
            .map(|y| draw_row(size, y, history))
            .collect::<Vec<_>>()
    ]
}

fn draw_row(size: u32, y: u32, history: &History) -> El<Msg> {
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
