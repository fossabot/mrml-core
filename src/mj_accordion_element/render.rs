use super::{MJAccordionElement, NAME};
use crate::helper::condition::negation_conditional_tag;
use crate::helper::tag::Tag;
use crate::mj_accordion_text::MJAccordionText;
use crate::mj_accordion_title::MJAccordionTitle;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Options, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

const CHILDREN_ATTRIBUTES: [&str; 9] = [
    "border",
    "icon-align",
    "icon-width",
    "icon-height",
    "icon-position",
    "icon-wrapped-url",
    "icon-wrapped-alt",
    "icon-unwrapped-url",
    "icon-unwrapped-alt",
];

struct MJAccordionElementRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJAccordionElement,
    extra: Map<String, String>,
}

impl<'e, 'h> MJAccordionElementRender<'e, 'h> {
    fn render_title(&self, opts: &Options) -> Result<String, Error> {
        if let Some(ref child) = self.element.children.title {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts)
        } else {
            let child = MJAccordionTitle::default();
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts)
        }
    }

    fn render_text(&self, opts: &Options) -> Result<String, Error> {
        if let Some(ref child) = self.element.children.text {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts)
        } else {
            let child = MJAccordionText::default();
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts)
        }
    }

    fn render_children(&self, opts: &Options) -> Result<String, Error> {
        Ok(self.render_title(opts)? + &self.render_text(opts)?)
    }
}

impl<'e, 'h> Render<'h> for MJAccordionElementRender<'e, 'h> {
    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra.insert(key.to_string(), value.to_string());
    }

    fn extra_attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.extra)
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &Options) -> Result<String, Error> {
        let input = negation_conditional_tag(
            Tag::new("input")
                .add_attribute("type", "checkbox")
                .add_class("mj-accordion-checkbox")
                .add_style("display", "none")
                .closed(),
        );
        let div = Tag::div().render(self.render_children(opts)?);
        let label = Tag::new("label")
            .add_class("mj-accordion-element")
            .add_style("font-size", "13px")
            .maybe_add_style("font-family", self.attribute("font-family"))
            .render(input + &div);
        let td = Tag::td()
            .add_style("padding", "0px")
            .maybe_add_style("background-color", self.attribute("background-color"))
            .render(label);
        let tr = Tag::tr()
            .maybe_add_class(self.attribute("css-class"))
            .render(td);
        Ok(tr)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJAccordionElement {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJAccordionElementRender::<'e, 'h> {
            element: self,
            header,
            extra: Map::new(),
        })
    }
}
