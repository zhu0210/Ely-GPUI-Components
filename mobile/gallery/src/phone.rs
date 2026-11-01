//! The gallery shaped for a phone: the chapters as a list, a chapter's page
//! full width, and Back to return. The desktop shell's 248 px sidebar leaves a
//! phone's page no room.

use ely_gpui_component::{
    buttons::{ButtonVariant, IconButton},
    primitives::IconName,
    theme::{ActiveTheme, ControlSize, Mode, TextSize, Theme},
};
use gpui::{
    Context, FocusHandle, FontWeight, KeyDownEvent, Render, ScrollHandle, Window, div, prelude::*,
    px,
};
use gpui_mobile::{StatusBarContentStyle, SystemChromeStyle};

use crate::pages;

pub struct Phone {
    /// The open chapter; the list when none.
    page: Option<usize>,
    focus: FocusHandle,
    list: ScrollHandle,
}

impl Phone {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            page: None,
            focus: cx.focus_handle(),
            list: ScrollHandle::new(),
        }
    }

    fn open(&mut self, page: usize, cx: &mut Context<Self>) {
        log::info!("gallery: page -> {}", pages::ALL[page].slug);
        self.page = Some(page);
        cx.notify();
    }

    /// Back to the list. False when already there, so Back leaves the app.
    fn back(&mut self, cx: &mut Context<Self>) -> bool {
        let was_open = self.page.take().is_some();
        cx.notify();
        was_open
    }

    fn header(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = cx.theme();
        let dark = theme.is_dark();
        let title = self.page.map_or("Ely Gallery", |page| pages::ALL[page].title);
        div()
            .flex()
            .flex_none()
            .items_center()
            .gap_2()
            .h(px(52.0))
            .px_2()
            .border_b_1()
            .border_color(theme.colors.border)
            .when(self.page.is_some(), |bar| {
                bar.child(
                    IconButton::new("back", IconName::ArrowLeft)
                        .tooltip("Back")
                        .variant(ButtonVariant::Ghost)
                        .on_click(cx.listener(|phone, _, _, cx| {
                            phone.back(cx);
                        })),
                )
            })
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .px_2()
                    .truncate()
                    .text_size(theme.text_size(TextSize::Lg))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(title),
            )
            .child(
                IconButton::new("theme", if dark { IconName::Sun } else { IconName::Moon })
                    .tooltip(if dark { "Light" } else { "Dark" })
                    .variant(ButtonVariant::Ghost)
                    .size(ControlSize::Md)
                    .on_click(cx.listener(move |_, _, _, cx| {
                        Theme::set_mode(if dark { Mode::Light } else { Mode::Dark }, cx);
                    })),
            )
    }

    fn list(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = cx.theme();
        let colors = &theme.colors;
        div()
            .id("chapters")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .track_scroll(&self.list)
            .children(pages::ALL.iter().enumerate().map(|(ix, page)| {
                div()
                    .id(page.slug)
                    .flex()
                    .items_center()
                    .gap_3()
                    .min_h(px(56.0))
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(colors.border)
                    .active(|style| style.bg(colors.hover))
                    .child(
                        div()
                            .w(px(24.0))
                            .font_family(theme.mono_family.clone())
                            .text_size(theme.text_size(TextSize::Xs))
                            .text_color(colors.fg_subtle)
                            .child(format!("{:02}", page.number)),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(div().text_color(colors.fg).child(page.title))
                            .child(
                                div()
                                    .text_size(theme.text_size(TextSize::Sm))
                                    .text_color(colors.fg_muted)
                                    .truncate()
                                    .child(page.summary),
                            ),
                    )
                    .on_click(cx.listener(move |phone, _, _, cx| phone.open(ix, cx)))
            }))
    }

    fn page(&self, ix: usize, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let page = &pages::ALL[ix];
        let summary = cx.theme().colors.fg_muted;
        // One id per chapter, so each keeps its own scroll position.
        div()
            .id(("page", ix))
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .child(
                div()
                    .p_4()
                    .pb(px(48.0))
                    .child(div().pb_4().text_color(summary).child(page.summary))
                    .child((page.render)(window, cx)),
            )
    }
}

impl Render for Phone {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if window.focused(cx).is_none() {
            self.focus.focus(window, cx);
        }
        let (top, bottom, left, right) = gpui_mobile::safe_area_insets();
        let theme = cx.theme();
        let (bg, fg) = (theme.colors.bg, theme.colors.fg);
        let (font, size) = (theme.font_family.clone(), theme.text_size(TextSize::Base));
        gpui_mobile::set_system_chrome(&SystemChromeStyle {
            status_bar_style: if theme.is_dark() {
                StatusBarContentStyle::Light
            } else {
                StatusBarContentStyle::Dark
            },
            ..Default::default()
        });
        let header = self.header(cx);
        let body = match self.page {
            Some(ix) => self.page(ix, window, cx).into_any_element(),
            None => self.list(cx).into_any_element(),
        };
        div()
            .track_focus(&self.focus)
            .size_full()
            .flex()
            .flex_col()
            .pt(px(top))
            .pb(px(bottom.max(gpui_mobile::keyboard_height())))
            .pl(px(left))
            .pr(px(right))
            .bg(bg)
            .text_color(fg)
            .font_family(font)
            .text_size(size)
            // Android's Back arrives as Escape; unhandled on the list, it leaves the app.
            .on_key_down(cx.listener(|phone, event: &KeyDownEvent, _, cx| {
                if event.keystroke.key == "escape" && phone.back(cx) {
                    cx.stop_propagation();
                }
            }))
            .child(header)
            .child(body)
    }
}
