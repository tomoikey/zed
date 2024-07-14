use crate::{Editor, EditorStyle};
use gpui::{
    div, AnyElement, ClickEvent, InteractiveElement, IntoElement, ParentElement, Pixels,
    RenderOnce, Size, Styled, ViewContext, WeakView, WindowContext,
};
use language::ParsedMarkdown;
use ui::{PopoverPage, StyledExt};
use workspace::Workspace;

#[derive(Debug, Default, PartialEq)]
pub struct SignatureHelpPopover {
    pub signature_help_markdowns: Vec<SignatureHelpMarkdown>,
    pub active_signature: usize,
}

#[derive(Clone, Debug)]
pub struct SignatureHelpMarkdown {
    pub signature: ParsedMarkdown,
    pub signature_description: Option<ParsedMarkdown>,
}

impl PartialEq for SignatureHelpMarkdown {
    fn eq(&self, other: &Self) -> bool {
        let signature_str_equality = self.signature.text.as_str() == other.signature.text.as_str();
        let signature_highlight_equality = self.signature.highlights == other.signature.highlights;

        let signature_description_str_equality = match (
            self.signature_description.as_ref(),
            other.signature_description.as_ref(),
        ) {
            (Some(text), Some(other_text)) => text.text.as_str() == other_text.text.as_str(),
            (None, None) => true,
            _ => false,
        };
        signature_str_equality && signature_highlight_equality && signature_description_str_equality
    }
}

impl SignatureHelpPopover {
    pub fn render(
        &mut self,
        style: &EditorStyle,
        max_size: Size<Pixels>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut ViewContext<Editor>,
    ) -> AnyElement {
        let pages = self
            .signature_help_markdowns
            .iter()
            .map(
                |SignatureHelpMarkdown {
                     signature,
                     signature_description,
                 }| {
                    let signature_element = div()
                        .id("signature_help_popover")
                        .max_w(max_size.width)
                        .child(div().p_2().child(crate::render_parsed_markdown(
                            "signature_help_popover_content",
                            signature,
                            style,
                            workspace.clone(),
                            cx,
                        )))
                        .into_any_element();
                    let boarder = div().border_primary(cx).border_1().into_any_element();

                    let children = if let Some(signature_description) = signature_description {
                        let signature_description_element = div()
                            .id("signature_help_popover_description")
                            .child(div().p_2().child(crate::render_parsed_markdown(
                                "signature_help_popover_description_content",
                                signature_description,
                                style,
                                workspace.clone(),
                                cx,
                            )))
                            .into_any_element();
                        vec![signature_element, boarder, signature_description_element]
                    } else {
                        vec![signature_element]
                    };

                    div()
                        .flex()
                        .flex_col()
                        .children(children)
                        .into_any_element()
                },
            )
            .collect();

        PopoverPage::new(
            pages,
            self.active_signature,
            |_: &ClickEvent, _: &mut WindowContext<'_>| {},
            |_: &ClickEvent, _: &mut WindowContext<'_>| {},
        )
        .render(cx)
        .into_any_element()
    }
}
