//! The release room: an inverted chat that opens mid-history. Scrolling
//! up loads older pages at the visual top; scrolling to the bottom loads
//! newer messages at the start, and the keys keep the visible messages in
//! place while they arrive.

use crate::ui::button::{ButtonSize, ButtonVariant, button_with};
use nsh_base::{input, theme};
use nsh_core::{element::SubmitBehavior, prelude::*};

use super::{heading, note};

/// What the messages say, in turn.
const LINES: [&str; 6] = [
    "Did the build go out?",
    "Yes, 2.4 is live on both stores",
    "Crash rate?",
    "Flat at 0.02 %",
    "The list scrolls so much smoother now",
    "Recycling plus native layout, no JS per frame",
];

/// How many messages a load brings, at either end.
const PAGE: u32 = 15;

/// The newest id the room's history reaches: loading newer stops here.
const NEWEST: u32 = 260;

/// The oldest id the room's history reaches: loading older stops here.
const OLDEST: u32 = 1;

/// The ids the chat opens with: 200 down to 81, mid-history, far enough
/// from both ends that opening loads nothing.
const FIRST_ID: u32 = 200;
const LAST_ID: u32 = 81;

/// One message. `id` orders the room: higher is newer.
#[derive(Clone, PartialEq)]
struct Message {
    id: u32,
    body: String,
    mine: bool,
}

/// A message of the room's history: its body from the script.
fn history(id: u32) -> Message {
    Message {
        id,
        body: LINES[(id as usize) % LINES.len()].to_owned(),
        mine: id.is_multiple_of(3),
    }
}

/// A bubble: mine at the end in the accent, theirs at the start in a card,
/// the script line with the room's id under it.
fn bubble(message: Memo<Message>) -> impl IntoElement {
    let mine = move || message.get().mine;
    let theirs = move || !mine();
    view()
        .style_when(mine, |v| v.self_end().items_end())
        .style_when(theirs, |v| v.self_start().items_start())
        .max_w(relative(0.78))
        .child(
            view()
                .px(px(14.0))
                .py(px(8.0))
                .rounded_lg()
                .style_when(mine, |v| v.bg(move || theme().primary).rounded_br(px(4.0)))
                .style_when(theirs, |v| {
                    v.bg(move || theme().card)
                        .border_1()
                        .border_color(move || theme().border)
                        .rounded_bl(px(4.0))
                })
                .child(
                    text(move || message.get().body.clone())
                        .text_sm()
                        .text_color(move || {
                            if mine() {
                                theme().primary_foreground
                            } else {
                                theme().card_foreground
                            }
                        }),
                ),
        )
        .child(
            text(move || format!("msg {}", message.get().id))
                .px_1()
                .text_xs()
                .tabular_nums()
                .text_color(move || theme().muted_foreground),
        )
}

/// The Chat demo: inverted, opened mid-history, loading at both ends.
#[component]
pub fn ChatDemo() -> impl IntoElement {
    // The room's ids the window reaches at either end; the loaders stop at
    // the room's ends. These drive the loaders, nothing displays them.
    let older_until = stored(LAST_ID);
    let newer_until = stored(FIRST_ID);
    let messages = signal((FIRST_ID..=NEWEST).rev().map(history).collect::<Vec<_>>());
    let draft = signal(String::new());
    // The room's newest id, grown by the Receive button once the start
    // loader has caught up.
    let received = stored(NEWEST);
    let receive = move |_: &()| {
        let id = received.with(|last| {
            *last += 1;
            *last
        });
        let body = LINES[(id as usize) % LINES.len()].to_owned();
        messages.update(|m| {
            m.insert(
                0,
                Message {
                    id,
                    body,
                    mine: false,
                },
            )
        });
    };
    let send = move || {
        let body = draft.get_untracked().trim().to_owned();
        if body.is_empty() {
            return;
        }
        draft.set(String::new());
        // A sent message is the newest: above whatever leads now.
        messages.update(|m| {
            let id = m.first().map_or(NEWEST, |m| m.id) + 1;
            m.insert(
                0,
                Message {
                    id,
                    body,
                    mine: true,
                },
            );
        });
    };
    // Scrolled to the visual top: older pages join at the end.
    let load_older = move |_: &()| {
        let last = older_until.with(|until| *until);
        if last <= OLDEST {
            return;
        }
        let first = (last - PAGE + 1).max(OLDEST);
        older_until.with(|until| *until = first);
        let loaded: Vec<_> = (first..=last).rev().map(history).collect();
        messages.update(|m| m.extend(loaded));
    };
    // Scrolled to the visual bottom: newer messages join at the start, and
    // the keys keep what is on screen where it is.
    let load_newer = move |_: &()| {
        let last = newer_until.with(|until| *until);
        if last >= NEWEST {
            return;
        }
        let next = (last + PAGE).min(NEWEST);
        newer_until.with(|until| *until = next);
        let loaded: Vec<_> = (last + 1..=next).rev().map(history).collect();
        messages.update(|m| {
            m.splice(0..0, loaded);
        });
    };
    let list = element_ref();
    let room = virtual_list(
        move || messages.with(Vec::len),
        move |at| {
            // The cell's message, read in a binding so the cell follows it.
            let message = computed(move || {
                messages.with(|m| {
                    m.get(at.index()).cloned().unwrap_or(Message {
                        id: 0,
                        body: String::new(),
                        mine: false,
                    })
                })
            });
            bubble(message)
        },
    )
    .test_id("chat-list")
    .keys(move || messages.with(|m| m.iter().map(|m| m.id).collect::<Vec<_>>()))
    .inverted()
    .estimated_item_size(px(56.0))
    .row_gap(px(6.0))
    .content_padding_x(px(4.0))
    .on_load_more(load_older)
    .on_load_more_at_start(load_newer)
    .flex_1()
    .element_ref(&list);
    view()
        .flex_1()
        .gap_2()
        .child(heading("Chat").child(note(
            "Inverted, opened mid-history; the ends load and stay put",
        )))
        .child(room)
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .pb_2()
                .child(button_with(
                    "Receive",
                    ButtonVariant::Secondary,
                    ButtonSize::Small,
                    false,
                    Some("Receive a message"),
                    move || receive(&()),
                ))
                .child(button_with(
                    "Latest",
                    ButtonVariant::Outline,
                    ButtonSize::Small,
                    false,
                    Some("Latest message"),
                    move || list.scroll_to_index(0, percentage(0.0), px(0.0), false),
                ))
                .child(
                    input()
                        .test_id("chat-draft")
                        .value(move || draft.get())
                        .on_change(move |text: &str| draft.set(text.to_owned()))
                        .on_submit(move |_: &str| send())
                        .submit_behavior(SubmitBehavior::Submit)
                        .placeholder("Message")
                        .placeholder_color(move || color(theme().muted_foreground))
                        .flex_1()
                        .h(px(40.0))
                        .px_3()
                        .text_sm()
                        .text_color(move || theme().foreground)
                        .bg(move || theme().card)
                        .border_1()
                        .border_color(move || theme().border)
                        .rounded_full(),
                )
                .child(button_with(
                    "↑",
                    ButtonVariant::Primary,
                    ButtonSize::Small,
                    false,
                    Some("Send"),
                    send,
                )),
        )
}
