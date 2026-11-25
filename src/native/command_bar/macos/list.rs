//! The command list: ranking against the search field, drawing the rows,
//! moving the highlight and running the chosen command.

use super::*;

impl Controller {
    /// Ranks the commands against the field and redraws the rows.
    pub(super) fn refilter(&self) {
        let field = self
            .ivars()
            .borrow()
            .views
            .as_ref()
            .map(|v| v.field.clone());
        // SAFETY: the field is an NSTextField.
        let query = field.map_or_else(String::new, |f| unsafe { string_value(&f) });
        {
            let mut state = self.ivars().borrow_mut();
            let entries: Vec<(String, String)> = state
                .items
                .iter()
                .map(|i| (i.title.clone(), i.group.clone()))
                .collect();
            state.shown = rank(&query, &entries);
            state.selected = 0;
        }
        self.draw_rows();
    }

    pub(super) fn draw_rows(&self) {
        let (list, shown, items) = {
            let state = self.ivars().borrow();
            let Some(views) = &state.views else { return };
            (views.list.clone(), state.shown.clone(), state.items.clone())
        };
        let mut rows = Vec::new();
        // SAFETY: NSView, NSBox, NSImageView, NSTextField and NSButton's
        // documented API; `list` is the flipped document view.
        unsafe {
            let old: Retained<AnyObject> = msg_send![&*list, subviews];
            let count: usize = msg_send![&*old, count];
            let old: Vec<Retained<AnyObject>> = (0..count)
                .map(|at| msg_send![&*old, objectAtIndex: at])
                .collect();
            for view in old {
                let _: () = msg_send![&*view, removeFromSuperview];
            }
            let visible = HEIGHT - TOP - FOOTER - 2.0;
            let height = (shown.len() as f64 * ROW + 12.0).max(visible);
            let _: () = msg_send![&*list, setFrameSize: NSSize::new(WIDTH, height)];
            if shown.is_empty() {
                let empty = label(
                    "No pages or actions match",
                    rect(24.0, 24.0, WIDTH - 48.0, 20.0),
                    14.0,
                    true,
                );
                let _: () = msg_send![&*list, addSubview: &*empty];
            }
            let w = WIDTH - 20.0;
            for (at, index) in shown.iter().enumerate() {
                let item = &items[*index];
                // NSBoxCustom: a rounded fill, shown on the selected row.
                let row: Retained<AnyObject> = msg_send![
                    msg_send![class(c"NSBox"), alloc],
                    initWithFrame: rect(10.0, 6.0 + at as f64 * ROW, w, ROW - 4.0)
                ];
                let _: () = msg_send![&*row, setBoxType: 4_usize];
                let _: () = msg_send![&*row, setTitlePosition: 0_usize];
                let _: () = msg_send![&*row, setBorderWidth: 0.0_f64];
                let _: () = msg_send![&*row, setCornerRadius: 10.0_f64];
                let _: () = msg_send![&*row, setContentViewMargins: NSSize::new(0.0, 0.0)];
                let label_color: Retained<AnyObject> = msg_send![class(c"NSColor"), labelColor];
                let fill: Retained<AnyObject> =
                    msg_send![&*label_color, colorWithAlphaComponent: 0.1_f64];
                let _: () = msg_send![&*row, setFillColor: &*fill];
                let _: () = msg_send![&*row, setTransparent: true];
                let body: Retained<AnyObject> = msg_send![&*row, contentView];
                if let Some(image) = symbol(&item.symbol) {
                    let icon: Retained<AnyObject> =
                        msg_send![class(c"NSImageView"), imageViewWithImage: &*image];
                    let _: () = msg_send![&*icon, setFrame: rect(14.0, 10.0, 20.0, 20.0)];
                    let _: () = msg_send![&*body, addSubview: &*icon];
                }
                let title = label(&item.title, rect(46.0, 11.0, w - 250.0, 18.0), 14.0, false);
                let _: () = msg_send![&*body, addSubview: &*title];
                let hint = if item.shortcut.is_empty() {
                    &item.group
                } else {
                    &item.shortcut
                };
                let hint = label(hint, rect(w - 200.0, 12.0, 184.0, 16.0), 12.0, true);
                let _: () = msg_send![&*hint, setAlignment: 1_isize];
                let _: () = msg_send![&*body, addSubview: &*hint];
                // A borderless, transparent button over the row: the
                // click.
                let hit: Retained<AnyObject> = msg_send![
                    msg_send![class(c"NSButton"), alloc],
                    initWithFrame: rect(0.0, 0.0, w, ROW - 4.0)
                ];
                let _: () = msg_send![&*hit, setTitle: &*ns_string("")];
                let _: () = msg_send![&*hit, setBordered: false];
                let _: () = msg_send![&*hit, setTransparent: true];
                let _: () = msg_send![&*hit, setTag: at as isize];
                let _: () = msg_send![&*hit, setTarget: self];
                let _: () = msg_send![&*hit, setAction: sel!(pick:)];
                let _: () = msg_send![&*body, addSubview: &*hit];
                let _: () = msg_send![&*list, addSubview: &*row];
                rows.push(row);
            }
        }
        self.ivars().borrow_mut().rows = rows;
        self.highlight();
    }

    pub(super) fn highlight(&self) {
        let (rows, selected, list) = {
            let state = self.ivars().borrow();
            let Some(views) = &state.views else { return };
            (state.rows.clone(), state.selected, views.list.clone())
        };
        // SAFETY: the rows are NSBoxes in the flipped list.
        unsafe {
            for (at, row) in rows.iter().enumerate() {
                let _: () = msg_send![&**row, setTransparent: at != selected];
            }
            if rows.get(selected).is_some() {
                let frame = rect(0.0, selected as f64 * ROW, WIDTH, ROW + 12.0);
                let _: bool = msg_send![&*list, scrollRectToVisible: frame];
            }
        }
    }

    pub(super) fn step(&self, by: isize) {
        {
            let mut state = self.ivars().borrow_mut();
            let count = state.shown.len() as isize;
            if count == 0 {
                return;
            }
            state.selected = (state.selected as isize + by).rem_euclid(count) as usize;
        }
        self.highlight();
    }

    /// Runs the listed command `at`: hides the bar, brings the main
    /// window forward for a destination, and hands the id to the app.
    pub(super) fn run(&self, at: usize) {
        let picked = {
            let state = self.ivars().borrow();
            state
                .shown
                .get(at)
                .and_then(|i| state.items.get(*i))
                .cloned()
                .zip(state.context.clone())
        };
        self.hide();
        let Some((item, context)) = picked else {
            return;
        };
        if item.front {
            self.show_main();
        }
        context.emit("run", serde_json::json!({ "id": item.id }));
    }
}
