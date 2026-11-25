//! The gallery's surface sections: overlays, data views, surfaces and
//! links.

use super::*;

pub(super) fn overlays_section(
    open_dialog: Signal<bool>,
    open_sheet: Signal<bool>,
    open_form_sheet: Signal<bool>,
    open_full_dialog: Signal<bool>,
    deletable: Signal<bool>,
) -> impl IntoElement {
    let menu_open = signal(false);
    let popover_open = signal(false);
    let show_hidden = signal(true);
    layout::section(
        "Overlays",
        view()
            .flex_col()
            .gap_4()
            .child(
                view()
                    .flex_row()
                    .flex_wrap()
                    .gap_2()
                    .child(popups::dropdown_menu(
                        "Actions",
                        [
                            popups::menu_entry("Rename…", || ()).into_element(),
                            popups::menu_entry("Duplicate", || ()).into_element(),
                            popups::checkable_menu_entry("Show hidden", show_hidden, move |next| {
                                show_hidden.set(next)
                            })
                            .into_element(),
                            popups::submenu_of(
                                "Move to…",
                                [
                                    popups::SubmenuEntry {
                                        label: "Projects",
                                        on_select: || {},
                                        destructive: false,
                                    },
                                    popups::SubmenuEntry {
                                        label: "Archive",
                                        on_select: || {},
                                        destructive: false,
                                    },
                                ],
                            )
                            .into_element(),
                            popups::menu_divider().into_element(),
                            popups::destructive_menu_entry("Delete…", || ()).into_element(),
                        ],
                    ))
                    .child(button::button_with(
                        "Filters sheet",
                        button::ButtonVariant::Outline,
                        button::ButtonSize::Medium,
                        false,
                        None,
                        move || open_sheet.set(true),
                    ))
                    .child(button::button_with(
                        "Delete draft…",
                        button::ButtonVariant::Danger,
                        button::ButtonSize::Medium,
                        move || !deletable.get(),
                        None,
                        move || {
                            dialog::confirm_destructive(
                                "Delete the draft?",
                                "This cannot be undone.",
                                "Delete",
                                move |confirmed| {
                                    if confirmed {
                                        toast::notify("Draft deleted");
                                    }
                                },
                            )
                        },
                    ))
                    .child(
                        button::button("Notify", || toast::notify("Copied to the clipboard"))
                            .into_element(),
                    ),
            )
            // Triggers hug their labels in one wrapping row; alone in the
            // column each stretched to the full width.
            .child(
                view()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap_2()
                    .child(popups::popover_of(
                        button::button_with(
                            "Details",
                            button::ButtonVariant::Outline,
                            button::ButtonSize::Medium,
                            false,
                            None,
                            move || popover_open.set(!popover_open.get()),
                        ),
                        view()
                            .w(px(220.0))
                            .flex_col()
                            .gap_1()
                            .child(text::body("Shipped yesterday"))
                            .child(text::muted("Popovers keep the task going.")),
                        move |open| popover_open.set(open),
                    ))
                    .child(
                        view()
                            .tooltip(popups::tooltip_of("Keyboard: mod+/"))
                            .child(button::icon_button("?", "Help", || ())),
                    )
                    .child(popups::hover_card_of(
                        button::button_with(
                            "Hover me",
                            button::ButtonVariant::Ghost,
                            button::ButtonSize::Medium,
                            false,
                            None,
                            || (),
                        ),
                        view()
                            .w(px(200.0))
                            .child(text::body("Ada Lovelace"))
                            .child(text::muted("Writes the first programs.")),
                    ))
                    .child(button::button_with(
                        "Command palette",
                        button::ButtonVariant::Secondary,
                        button::ButtonSize::Medium,
                        false,
                        None,
                        move || menu_open.set(true),
                    )),
            )
            // Commands on the object under the pointer or finger: right
            // click on the desktops, a held finger on touch screens.
            .child(popups::context_menu_of(
                layout::card().child(text::body(
                    "Right-click (or hold) this card for its commands.",
                )),
                [
                    popups::menu_entry("Duplicate card", || ()).into_element(),
                    popups::menu_divider().into_element(),
                    popups::destructive_menu_entry("Delete card…", || ()).into_element(),
                ],
            ))
            .child(command::command_palette(
                vec![
                    command::command_entry("open.gallery", "Open the UI gallery"),
                    command::command_entry("toggle.dark", "Toggle dark appearance"),
                    command::command_entry("copy.id", "Copy the app id"),
                ],
                "Type a command…",
                move |_| toast::notify("Command run"),
            ))
            .child(
                view()
                    .flex_row()
                    .flex_wrap()
                    .gap_2()
                    .child(button::button_with(
                        "Open dialog",
                        button::ButtonVariant::Outline,
                        button::ButtonSize::Small,
                        false,
                        None,
                        move || open_dialog.set(true),
                    ))
                    .child(button::button_with(
                        "Open page sheet",
                        button::ButtonVariant::Outline,
                        button::ButtonSize::Small,
                        false,
                        None,
                        move || open_form_sheet.set(true),
                    ))
                    .child(button::button_with(
                        "Open full dialog",
                        button::ButtonVariant::Outline,
                        button::ButtonSize::Small,
                        false,
                        None,
                        move || open_full_dialog.set(true),
                    )),
            ),
    )
}

pub(super) fn data_section() -> impl IntoElement {
    let page = signal(2_u32);
    let expanded = signal(vec![SharedString::from("src")]);
    let selected = signal(SharedString::from("src/ui"));
    layout::section(
        "Data views",
        view()
            .flex_col()
            .gap_4()
            .child(table::table_of(
                &["Invoice", "Status", "Total"],
                &[
                    vec!["INV001".into(), "Paid".into(), "$120.00".into()],
                    vec!["INV002".into(), "Due".into(), "$80.50".into()],
                ],
            ))
            .child(
                table::data_table_of(
                    vec![
                        table::column("symbol", "Symbol", 120.0),
                        table::column("last", "Last", 90.0),
                        table::column("change", "Change", 90.0),
                    ],
                    nsh_base::DataTableRows::new(2)
                        .texts("symbol", ["AAPL", "MSFT"])
                        .numbers("last", [231.5, 412.0])
                        .numbers("change", [1.2, -0.4]),
                )
                .h(px(160.0)),
            )
            .child(tree::tree_of(
                vec![tree::tree_row(
                    "src",
                    "src",
                    vec![
                        tree::tree_row("src/ui", "ui", vec![]),
                        tree::tree_row("src/shell.rs", "shell.rs", vec![]),
                    ],
                )],
                expanded,
                selected,
            ))
            .child(pagination::pagination_of(page, 12))
            .child(text::muted(move || format!("Page {} of 12", page.get()))),
    )
}

pub(super) fn surfaces_section() -> impl IntoElement {
    let open_details = signal(true);
    let accordion = signal(vec![SharedString::from("billing")]);
    let page = signal(0_u32);
    let sizes = signal(Vec::new());
    let row_sizes = signal(Vec::new());
    let layout_signal = signal(DockAreaLayout::new(
        DockLayout::h_split()
            .child(DockLayout::tabs().panel("files"), Some(px(200.0)))
            .child(DockLayout::tabs().panel("editor"), None),
    ));
    layout::section(
        "Surfaces",
        view()
            .flex_col()
            .gap_4()
            .child(layout::card().child(text::body("A card: the raised surface.")))
            .child(collapse::collapsible_section(
                "Details",
                open_details,
                view()
                    .flex_col()
                    .gap_1()
                    .child(text::body("Opened with a springing reveal."))
                    .child(text::muted("State inside survives reopenings.")),
            ))
            .child(collapse::accordion_of(
                &[
                    collapse::AccordionEntry {
                        id: "shipping",
                        title: "Shipping",
                        content: "Two to four days.".into(),
                    },
                    collapse::AccordionEntry {
                        id: "billing",
                        title: "Billing",
                        content: "Invoiced monthly.".into(),
                    },
                ],
                accordion,
                false,
            ))
            .child(
                carousel::carousel_of(
                    "Gallery",
                    vec![
                        text::body("First slide").into_element(),
                        text::body("Second slide").into_element(),
                        text::body("Third slide").into_element(),
                    ],
                    page,
                )
                .h(px(120.0)),
            )
            .child(
                resizable::v_resizable_panels(
                    vec![
                        text::body("Top pane").into_element(),
                        text::muted("Drag the handle").into_element(),
                    ],
                    sizes,
                )
                .h(px(140.0)),
            )
            .child(
                resizable::h_resizable_panels(
                    vec![
                        text::body("Leading pane").into_element(),
                        text::muted("Drag the handle").into_element(),
                    ],
                    row_sizes,
                )
                .h(px(120.0)),
            )
            .child(
                dock::dock_workspace(
                    vec![
                        dock::DockPanelSpec {
                            key: "files",
                            title: "Files",
                            content: "main.rs".into(),
                        },
                        dock::DockPanelSpec {
                            key: "editor",
                            title: "Editor",
                            content: "The workspace.".into(),
                        },
                    ],
                    layout_signal,
                )
                .h(px(220.0))
                .rounded_lg()
                .border_1()
                .border_color(move || theme().border)
                .overflow_hidden(),
            )
            .child(field::field(
                "Notes",
                editor::editor_of(
                    "Notes",
                    signal(SharedString::from("fn main() {}\n")),
                    "rust",
                ),
            ))
            .child(editor::rich_editor_of(
                serde_json::json!({
                    "type": "doc",
                    "content": [
                        { "type": "paragraph", "content": [
                            { "type": "text", "text": "Write something. " },
                            { "type": "text", "marks": [{ "type": "bold" }], "text": "Bold works." }
                        ]}
                    ]
                }),
                "Write something…",
            ))
            .child(editor::diff_view_of(
                "fn main() {}\n".into(),
                "fn main() {\n    println!(\"hi\");\n}\n".into(),
            ))
            .child(editor::markdown_of(
                "# Notes\n\nA **document** with `code`, rendered natively.".to_owned(),
            )),
    )
}

pub(super) fn link_section() -> impl IntoElement {
    let rail = element_ref();
    layout::section(
        "Link and scroll bar",
        view()
            .flex_col()
            .gap_4()
            .child(link::link_out("https://nsh.dev", "Read the NSH docs"))
            .child(
                view()
                    .relative()
                    .h(px(120.0))
                    .child(
                        scroll().flex_1().scrollbar(false).element_ref(&rail).child(
                            view()
                                .flex_col()
                                .gap_1()
                                .p_3()
                                .children((1..=30).map(|line| {
                                    text::body(format!("Line {line} of the scrolled content"))
                                })),
                        ),
                    )
                    .child(
                        view()
                            .absolute()
                            .top_0()
                            .right_0()
                            .bottom_0()
                            .child(crate::ui::scrollbar::scrollbar_for(rail, "Content")),
                    ),
            ),
    )
}
