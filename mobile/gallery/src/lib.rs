//! The gallery's pages on gpui-mobile. The pages, shell and stories are the
//! desktop gallery's own files; the desktop capture harness stays behind.
// The gallery's modules are written for a binary, and the mobile shell uses part of them.
#![allow(unreachable_pub, dead_code)]

/// A file under the gallery's assets, by name.
macro_rules! asset {
    ($name:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../crates/ely-gallery/assets/", $name)
    };
}

#[path = "../../../crates/ely-gallery/src/assets.rs"]
mod assets;
#[path = "../../../crates/ely-gallery/src/pages/mod.rs"]
mod pages;
#[path = "../../../crates/ely-gallery/src/probe.rs"]
mod probe;
#[path = "../../../crates/ely-gallery/src/shell.rs"]
mod shell;
#[path = "../../../crates/ely-gallery/src/step.rs"]
mod step;
#[path = "../../../crates/ely-gallery/src/ui.rs"]
mod ui;

use anyhow::{Context as _, Result};
use gpui::{App, AppContext, Menu, MenuItem, SharedString, WindowHandle, WindowOptions};

gpui::actions!(gallery, [LightMode, DarkMode]);

/// The menus MenuBar draws; a phone has no menu bar of its own.
pub fn app_menus() -> Vec<Menu> {
    vec![Menu {
        name: "View".into(),
        items: vec![
            MenuItem::action("Light", LightMode),
            MenuItem::action("Dark", DarkMode),
        ],
        disabled: false,
    }]
}

/// What the gallery opens on.
#[derive(Default)]
pub struct Start {
    pub page: usize,
    /// One section, by title or slug, drawn alone.
    pub story: Option<SharedString>,
    pub choice: shell::Choice,
    /// Narrow check width, if any.
    pub narrow: Option<f32>,
    /// The window's size when not the usual 1280 by 820.
    pub window: Option<gpui::Size<gpui::Pixels>>,
    /// Why the start could not be read, shown instead.
    pub failure: Option<SharedString>,
}

/// Sets Ely up and opens the gallery full screen.
fn launch(cx: &mut App) -> Result<WindowHandle<shell::Gallery>> {
    ely_gpui_component::init(cx)?;
    pages::bind_keys(cx);
    pages::install_i18n(cx);
    let window = cx
        .open_window(WindowOptions::default(), |window, cx| {
            cx.new(|cx| shell::Gallery::new(Start::default(), window, cx))
        })
        .context("gallery window failed to open")?;
    cx.on_action(move |_: &LightMode, cx| choose(window, shell::Choice::Light, cx));
    cx.on_action(move |_: &DarkMode, cx| choose(window, shell::Choice::Dark, cx));
    Ok(window)
}

fn choose(window: WindowHandle<shell::Gallery>, choice: shell::Choice, cx: &mut App) {
    if let Err(error) = window.update(cx, |gallery, window, cx| gallery.choose(choice, window, cx))
    {
        log::error!("gallery: menu could not reach the window: {error:#}");
    }
}

/// Called by `android-activity` on its own thread; returns when the Activity ends.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    use gpui_mobile::android::jni;

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("ely-gallery"),
    );
    jni::install_panic_hook();
    jni::init_platform(&app);
    let Some(platform) = jni::shared_platform() else {
        log::error!("gallery: gpui-mobile has no platform");
        return;
    };
    gpui::Application::with_platform(platform.into_rc())
        .with_assets(assets::GalleryAssets)
        .run(|cx: &mut App| {
            if let Err(error) = launch(cx) {
                log::error!("gallery: {error:#}");
            }
        });
}
