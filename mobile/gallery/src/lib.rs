//! The gallery's pages on gpui-mobile. The pages, shell and stories are the
//! desktop gallery's own files; the desktop capture harness stays behind.
// The gallery's modules are written for a binary, and the mobile shell uses part of them.
#![allow(unreachable_pub, dead_code)]

/// A file under the gallery's assets, by name.
macro_rules! asset {
    ($name:literal) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/ely-gallery/assets/",
            $name
        )
    };
}

#[path = "../../../crates/ely-gallery/src/assets.rs"]
mod assets;
#[path = "../../../crates/ely-gallery/src/pages/mod.rs"]
mod pages;
mod phone;
#[path = "../../../crates/ely-gallery/src/probe.rs"]
mod probe;
#[path = "../../../crates/ely-gallery/src/step.rs"]
mod step;
#[path = "../../../crates/ely-gallery/src/ui.rs"]
mod ui;

use anyhow::{Context as _, Result};
use ely_gpui_component::theme::{Mode, Theme};
use gpui::{App, AppContext, Menu, MenuItem, WindowOptions};

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

/// Sets Ely up and opens the gallery full screen.
fn launch(cx: &mut App) -> Result<()> {
    ely_gpui_component::init(cx)?;
    pages::bind_keys(cx);
    pages::install_i18n(cx);
    cx.on_action(|_: &LightMode, cx| Theme::set_mode(Mode::Light, cx));
    cx.on_action(|_: &DarkMode, cx| Theme::set_mode(Mode::Dark, cx));
    // MenuBar draws them; a phone has no menu bar of its own.
    cx.set_menus(app_menus());
    cx.open_window(WindowOptions::default(), |_, cx| cx.new(phone::Phone::new))
        .context("gallery window failed to open")?;
    Ok(())
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

/// Called by the iOS host's app delegate while launching, after it hands over
/// the launch link; returns once the window is open, and frames come from the
/// host's display link.
#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub extern "C" fn ely_gallery_ios_start() {
    gpui_mobile::ios::ffi::set_app_callback(Box::new(|cx: &mut App| {
        if let Err(error) = launch(cx) {
            log::error!("gallery: {error:#}");
        }
    }));
    gpui_mobile::ios::ffi::run_app_with_assets(assets::GalleryAssets);
}
