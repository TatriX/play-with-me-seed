use crate::{History, Model, Msg, Stage, TOKENS};
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
        label![
            "Name",
            input![
                attrs! {At::Value => model.player, At::AutoFocus => true},
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
                                class![if token == &model.token {
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
                button!["Refresh", simple_ev(Ev::Click, Msg::CleanHistory)],
            ],
            label![
                "Session",
                input![attrs! {At::Value => model.session, At::ReadOnly => true }]
            ],
        ],
        hr![],
        div![
            id!("gameplay-area"),
            player_list(&model.history.players, &model.player),
            draw_grid(model.size, &model.history),
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

fn player_list(players: &[String], player: &str) -> El<Msg> {
    ul![
        id!("players"),
        li![class!["list-header"], "Players:"],
        players
            .iter()
            .map(|name| li![class![if name == player { "is-player" } else { "" }], name])
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
                let content = history
                    .moves
                    .iter()
                    .find(|cell| cell.coord.row == y && cell.coord.col == x)
                    .map(|cell| cell.value.clone())
                    .unwrap_or(" ".into());
                div![
                    id!(&format!("cell-{}-{}", x, y)),
                    class!["cell"],
                    simple_ev(Ev::Click, Msg::Move { x, y }),
                    content,
                ]
            })
            .collect::<Vec<_>>()
    ]
}
