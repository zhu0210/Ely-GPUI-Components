use gpui::{HitboxBehavior, IntoElement, Styled, TouchDragEvent, TouchPhase, canvas};

/// Claims a finger's drag over its parent from the first move, so a slider's thumb or a handle
/// follows the finger instead of the page scrolling. gpui offers the drag as the finger lands,
/// before it shows a direction, so a page cannot scroll across what claims it: keep it to small
/// targets. The event goes on, for the window to replay as a mouse drag (gpui-mobile's
/// `touch_drags`). The parent must be positioned.
pub(crate) fn touch_grip() -> impl IntoElement {
    canvas(
        |bounds, window, _| window.insert_hitbox(bounds, HitboxBehavior::Normal),
        |_, hitbox, window, _| {
            window.on_mouse_event(move |event: &TouchDragEvent, phase, window, _| {
                if phase.bubble() && event.phase == TouchPhase::Started && hitbox.is_hovered(window)
                {
                    window.prevent_default();
                }
            });
        },
    )
    .absolute()
    .size_full()
}
