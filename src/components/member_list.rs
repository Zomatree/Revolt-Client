use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct MemberListProps {
    pub channel_id: types::ULID,
    pub server_id: types::ULID
}

pub fn MemberList<'a>(cx: Scope<'a, MemberListProps>) -> Element<'a> {
    let channel_state = use_read(&cx, CHANNELS);
    let member_state = use_read(&cx, SERVER_MEMBERS);
    let user_state = use_read(&cx, USERS);
    let revolt_config = use_read(&cx, REVOLT_CONFIG).as_ref().unwrap();
    let member_list = &member_state[&cx.props.server_id];

    cx.render(rsx! {
        div {
            style: "width: 232px; height: 100%; overflow-y: auto",

            member_list.keys().map(|member_id| {
                let user = &user_state[member_id];
                let (username, avatar) = utils::get_username_avatar(channel_state, member_state, revolt_config, user, &None, Some(&cx.props.channel_id));

                rsx! {
                    div {
                        components::Icon {
                            src: avatar
                        },
                        span {
                            "{username}"
                        }
                    }
                }
            })
        }
    })
}
