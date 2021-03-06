use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct MessageProps {
    pub message_id: types::ULID,
    pub channel_id: types::ULID
}

pub fn Message(cx: Scope<MessageProps>) -> Element {
    let message_state = use_read(&cx, MESSAGES);
    let channel_state = use_read(&cx, CHANNELS);
    let server_members = use_read(&cx, SERVER_MEMBERS);
    let revolt_config = use_read(&cx, REVOLT_CONFIG).as_ref().unwrap();
    let modal = utils::use_modal(&cx);

    let message = message_state
        .get(&cx.props.channel_id)?
        .get(&cx.props.message_id)?;

    let types::Message { content, author, attachments, channel, masquerade, replies, edited, id, .. } = message;

    let user_state = use_read(&cx, USERS);
    let user = user_state.get(author).unwrap();
    let (username, avatar) = get_username_avatar(channel_state, server_members, revolt_config, user, masquerade, Some(channel));
    let content = content.clone().unwrap_or_default();
    let created_at = cx.use_hook(|_| format_datetime(&id.timestamp()));  // only needs to be calculated once

    cx.render(rsx! {
        div {
            style: "display: flex; padding: 0.125rem; margin-top: 12px; padding-inline-end: 16px; flex-direction: column",
            div {
                style: "display: flex; flex-direction: column",
                replies
                    .iter()
                    .map(|reply| {
                        rsx! {
                            div {
                                style: "gap: 8px; min-width: 8px; display: flex; margin-inline: 30px 12px; font-size: 0.8em",
                                key: "{reply}",
                                components::Reply {
                                    message_id: reply.clone(),
                                    channel_id: cx.props.channel_id.clone(),
                                    message_mentions: replies.clone()
                                }
                            }
                        }
                    })
            },
            div {
                style: "display: flex; flex-direction: row",
                div {
                    style: "display: flex; flex-direction: row; width: 62px",
                    img {
                        src: "{avatar}",
                        width: "44",
                        height: "44"
                    },
                },
                div {
                    style: "display: flex; flex-direction: column; justify-content: center; flex-grow: 1",
                    span {
                        style: "gap: 8px; display: flex; align-items: center",
                        span { "{username}" },
                        user.bot.is_some().then(|| rsx! {
                            span {
                                "[BOT]"
                            }
                        }),
                        time { "{created_at}" },
                        edited.is_some().then(|| rsx! {
                            span {
                                style: "font-size: 10px",
                                "(edited)"
                            }
                        })
                    },
                    components::Markdown {
                        text: content
                    }
                },
            }
            attachments.iter().cloned().enumerate().map(|(i, asset)| {
                rsx! {
                    div {
                        key: "{i}",
                        components::Attachment {
                            asset: asset
                        }
                    }
                }
            }),
            button {
                onclick: move |_| {
                    modal.push_modal(utils::ActiveModal::DeleteMessage {
                        channel_id: cx.props.channel_id.clone(),
                        message_id: cx.props.message_id.clone()
                    })
                },
                "delete"
            }

        }
    })
}
