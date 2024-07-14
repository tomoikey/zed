use crate::{Button, ButtonCommon, ButtonSize, Clickable, Disableable, Label, StyledExt};
use gpui::{div, AnyElement, IntoElement, ParentElement, RenderOnce, Styled, WindowContext};

pub struct PopoverPage {
    pages: Vec<AnyElement>,
    current: usize,
    on_click_previous: Box<dyn Fn(&gpui::ClickEvent, &mut WindowContext) + 'static>,
    on_click_next: Box<dyn Fn(&gpui::ClickEvent, &mut WindowContext) + 'static>,
}

impl PopoverPage {
    pub fn new<F, G>(
        pages: Vec<AnyElement>,
        current: usize,
        on_click_previous: F,
        on_click_next: G,
    ) -> Self
    where
        F: Fn(&gpui::ClickEvent, &mut WindowContext) + 'static,
        G: Fn(&gpui::ClickEvent, &mut WindowContext) + 'static,
    {
        Self {
            pages,
            current,
            on_click_previous: Box::new(on_click_previous),
            on_click_next: Box::new(on_click_next),
        }
    }

    pub fn has_next_page(&self) -> bool {
        self.current + 1 < self.pages.len()
    }

    pub fn has_previous_page(&self) -> bool {
        self.current > 0
    }

    pub fn add_page(&mut self, page: AnyElement) {
        self.pages.push(page);
    }

    pub fn next_page(&mut self) -> bool {
        if self.has_next_page() {
            self.current += 1;
            true
        } else {
            false
        }
    }

    pub fn previous_page(&mut self) -> bool {
        if self.has_previous_page() {
            self.current -= 1;
            true
        } else {
            false
        }
    }
}

impl RenderOnce for PopoverPage {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let mut popover_page = self;

        let disabled_previous_button = !popover_page.has_previous_page();
        let disabled_next_button = !popover_page.has_next_page();

        let on_click_previous = popover_page.on_click_previous;
        let on_click_next = popover_page.on_click_next;

        // `remove` may panic if the current page is out of bounds
        if popover_page.pages.get(popover_page.current).is_none() {
            return div().into_any_element();
        }

        // after removing the current page, the length of the pages vector will be reduced by 1
        let length = popover_page.pages.len();

        let current_page = popover_page
            .pages
            .remove(popover_page.current)
            .into_any_element();
        let current_page = div().child(current_page).into_any_element();

        if length > 1 {
            let previous_button = div().flex().flex_row().justify_center().child(
                Button::new("popover_page_button_previous", "↑")
                    .size(ButtonSize::Compact)
                    .disabled(disabled_previous_button)
                    .on_click(move |event, cx| {
                        on_click_previous(event, cx);
                    })
                    .into_any_element(),
            );
            let page = div()
                .flex()
                .flex_row()
                .justify_center()
                .child(Label::new(format!(
                    "{} / {}",
                    popover_page.current + 1,
                    length
                )));
            let next_button = div().flex().flex_row().justify_center().child(
                Button::new("popover_page_button_next", "↓")
                    .size(ButtonSize::Compact)
                    .disabled(disabled_next_button)
                    .on_click(move |event, cx| {
                        on_click_next(event, cx);
                    })
                    .into_any_element(),
            );
            let buttons = div()
                .flex()
                .child(div().p_1().flex().flex_col_reverse().children([
                    next_button,
                    page,
                    previous_button,
                ]))
                .into_any_element();

            let boarder = div().border_primary(cx).border_1().into_any_element();

            div()
                .elevation_2(cx)
                .flex()
                .flex_row()
                .children([buttons, boarder, current_page])
                .into_any_element()
        } else {
            div().elevation_2(cx).child(current_page).into_any_element()
        }
    }
}
