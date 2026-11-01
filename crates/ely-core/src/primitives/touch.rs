use gpui::{Axis, HitboxBehavior, IntoElement, Styled, TouchDragEvent, TouchPhase, canvas};

/// Claims a finger's drag over its parent from the first move, so a slider or a handle follows
/// the finger instead of the page scrolling; with `along`, only a drag that starts mostly along
/// that axis, so a page still scrolls across the control. The event goes on, for the window to
/// replay as a mouse drag (gpui-mobile's `touch_drags`). The parent must be positioned.
pub(crate) fn touch_grip(along: Option<Axis>) -> impl IntoElement {
    canvas(
        |bounds, window, _| window.insert_hitbox(bounds, HitboxBehavior::Normal),
        move |_, hitbox, window, _| {
            window.on_mouse_event(move |event: &TouchDragEvent, phase, window, _| {
                let moved = event.position - event.start_position;
                let sideways = moved.x.abs() > moved.y.abs();
                let on_axis = match along {
                    None => true,
                    Some(Axis::Horizontal) => sideways,
                    Some(Axis::Vertical) => !sideways,
                };
                if phase.bubble()
                    && event.phase == TouchPhase::Started
                    && on_axis
                    && hitbox.is_hovered(window)
                {
                    window.prevent_default();
                }
            });
        },
    )
    .absolute()
    .size_full()
}
