use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct ChannelProps {
    channel_id: types::ULID,
    server_id: types::ULID
}

pub fn Channel(cx: Scope<ChannelProps>) -> Element {
    let message_state = use_read(&cx, MESSAGES);

    let mut messages = message_state
        .get(&cx.props.channel_id)
        .cloned()
        .unwrap_or_default()
        .values()
        .cloned()
        .collect::<Vec<_>>();

    messages.sort_by(|a, b| a.id.timestamp().cmp(&b.id.timestamp()));

    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 100%; flex-grow: 1",
            div {
                style: "background-color: grey; overflow-y: auto; flex-grow: 1",
                messages.into_iter().map(|msg| {
                    let message_id = msg.id.clone();

                    rsx! {
                        div {
                            key: "{message_id}",
                            components::Message {
                                channel_id: msg.channel,
                                message_id: message_id.clone(),
                            }
                        }
                    }
                })
            }
        }
    })
}