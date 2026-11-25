//! The app's main window, as the command bar needs it: brought forward
//! for a destination, and kept alive when its close button is pressed.

use super::*;

impl Controller {
    pub(super) fn main_window(&self) -> Option<Retained<AnyObject>> {
        let bridge = self.ivars().borrow().bridge.clone()?;
        // SAFETY: NSView's `window`.
        unsafe { msg_send![&*bridge, window] }
    }

    pub(super) fn show_main(&self) {
        let Some(window) = self.main_window() else {
            return;
        };
        let none: Option<&AnyObject> = None;
        // SAFETY: NSApplication and NSWindow's documented API.
        unsafe {
            let app: Retained<AnyObject> = msg_send![class(c"NSApplication"), sharedApplication];
            let _: () = msg_send![&*app, activateIgnoringOtherApps: true];
            let minimized: bool = msg_send![&*window, isMiniaturized];
            if minimized {
                let _: () = msg_send![&*window, deminiaturize: none];
            }
            let _: () = msg_send![&*window, makeKeyAndOrderFront: none];
        }
    }

    /// The main window's close button and Cmd+W hide it instead: the
    /// runner quits when its window closes (`keep_alive_on_close` is
    /// not reachable from an app), and the menu bar item must outlive
    /// it. The Dock icon and "Show nsh-starter" bring it back.
    pub(super) fn patch_main(&self) {
        if self.ivars().borrow().main_patched {
            return;
        }
        let Some(window) = self.main_window() else {
            // Not in its window yet: try again shortly.
            let none: Option<&AnyObject> = None;
            // SAFETY: NSObject's `performSelector:withObject:afterDelay:`.
            let _: () = unsafe {
                msg_send![
                    self,
                    performSelector: sel!(patchMain:),
                    withObject: none,
                    afterDelay: 0.25_f64
                ]
            };
            return;
        };
        // SAFETY: NSWindow, NSButton, NSApplication and NSMenu's
        // documented API.
        unsafe {
            // NSWindowCloseButton.
            let close: Option<Retained<AnyObject>> =
                msg_send![&*window, standardWindowButton: 0_usize];
            if let Some(close) = close {
                let _: () = msg_send![&*close, setTarget: self];
                let _: () = msg_send![&*close, setAction: sel!(hideMain:)];
            }
            let app: Retained<AnyObject> = msg_send![class(c"NSApplication"), sharedApplication];
            let main: Option<Retained<AnyObject>> = msg_send![&*app, mainMenu];
            if let Some(main) = main {
                let tops: Retained<AnyObject> = msg_send![&*main, itemArray];
                let count: usize = msg_send![&*tops, count];
                for at in 0..count {
                    let top: Retained<AnyObject> = msg_send![&*tops, objectAtIndex: at];
                    let menu: Option<Retained<AnyObject>> = msg_send![&*top, submenu];
                    let Some(menu) = menu else { continue };
                    let entries: Retained<AnyObject> = msg_send![&*menu, itemArray];
                    let n: usize = msg_send![&*entries, count];
                    for i in 0..n {
                        let entry: Retained<AnyObject> = msg_send![&*entries, objectAtIndex: i];
                        let action: Option<Sel> = msg_send![&*entry, action];
                        if action == Some(sel!(performClose:)) {
                            let _: () = msg_send![&*entry, setTarget: self];
                            let _: () = msg_send![&*entry, setAction: sel!(hideMain:)];
                        }
                    }
                }
            }
        }
        self.ivars().borrow_mut().main_patched = true;
    }
}
