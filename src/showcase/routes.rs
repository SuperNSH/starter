//! The demo screens' addresses. `Route::Showcase` in `src/routes.rs` wraps
//! this enum, so the showcase leaves the app as one variant.

/// Where a demo screen lives. A screen pushed from another starts with its
/// section's name (`DataItem` belongs to `data`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Gallery,
    NativeVsApp,
    Forms,
    Data,
    DataItem(u64),
    DataDevtools,
    Lists,
    Canvas,
    Motion,
    Overlays,
    Platform,
    PlatformDevice,
    RemoteScreen,
}

impl Route {
    /// The id the route carries.
    pub fn id(self) -> Option<u64> {
        match self {
            Route::DataItem(id) => Some(id),
            _ => None,
        }
    }

    /// The route a pushed screen sits on.
    pub fn parent(self) -> Option<Route> {
        match self {
            Route::DataItem(_) => Some(Route::Data),
            _ => None,
        }
    }

    /// The route's link path, without the scheme.
    pub fn path(self) -> String {
        match self {
            Route::Gallery => "gallery".into(),
            Route::NativeVsApp => "native_vs_app".into(),
            Route::Forms => "forms".into(),
            Route::Data => "data".into(),
            Route::DataItem(id) => format!("data/item/{id}"),
            Route::DataDevtools => "data/devtools".into(),
            Route::Lists => "lists".into(),
            Route::Canvas => "canvas".into(),
            Route::Motion => "motion".into(),
            Route::Overlays => "overlays".into(),
            Route::Platform => "platform".into(),
            Route::PlatformDevice => "platform/device".into(),
            Route::RemoteScreen => "remote_screen".into(),
        }
    }

    /// The route a link path names. `None` when it is not a demo screen's.
    pub fn parse(segments: &[&str]) -> Option<Route> {
        match segments {
            ["gallery"] => Some(Route::Gallery),
            ["native_vs_app"] => Some(Route::NativeVsApp),
            ["forms"] => Some(Route::Forms),
            ["data"] => Some(Route::Data),
            ["data", "item", id] => Some(Route::DataItem(id.parse().ok()?)),
            ["data", "devtools"] => Some(Route::DataDevtools),
            ["lists"] => Some(Route::Lists),
            ["canvas"] => Some(Route::Canvas),
            ["motion"] => Some(Route::Motion),
            ["overlays"] => Some(Route::Overlays),
            ["platform"] => Some(Route::Platform),
            ["platform", "device"] => Some(Route::PlatformDevice),
            ["remote_screen"] => Some(Route::RemoteScreen),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_path_parses_back_into_its_route() {
        let routes = [
            Route::Gallery,
            Route::NativeVsApp,
            Route::Forms,
            Route::Data,
            Route::DataItem(3),
            Route::DataDevtools,
            Route::Lists,
            Route::Canvas,
            Route::Motion,
            Route::Overlays,
            Route::Platform,
            Route::PlatformDevice,
            Route::RemoteScreen,
        ];
        for route in routes {
            let path = route.path();
            let segments: Vec<&str> = path.split('/').collect();
            assert_eq!(Route::parse(&segments), Some(route), "{path}");
        }
        assert_ne!(Route::DataItem(3).path(), Route::DataItem(4).path());
        assert_eq!(Route::parse(&["data", "item", "x"]), None);
    }
}
