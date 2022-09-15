use super::SvgListItemRender;
use crate::{
    component::Component,
    render::{
        base::{BaseElementRender, ElementRenderMut, MakeNodesExtensions, NodesExtensions},
        svg::{
            SvgAttributesOnly, SvgElementRender, SvgStaticAttributes, SvgStaticAttributesOnly,
            SvgTag,
        },
        ListElementCreation,
    },
};

pub trait SemsForList<'a, C: Component>:
    Sized + ElementRenderMut<C> + MakeNodesExtensions<'a>
{
    fn list_with_render<I, II, R>(
        mut self,
        items: II,
        mode: ListElementCreation,
        tag: &'static str,
        render: R,
    ) -> NodesExtensions<'a>
    where
        II: Iterator<Item = I>,
        for<'r> R: Fn(&I, crate::Svg<'r, C>),
    {
        let (comp, state, mut r) = self.element_render_mut().list_render(mode);
        let _do_we_have_to_care_about_this_returned_value_ = r.render(
            comp,
            state,
            items,
            SvgTag(tag),
            |item: &I, er: BaseElementRender<C>| render(item, er.into()),
        );
        self.make_nodes_extensions()
    }

    fn lwr_clone<I, II, R>(self, items: II, tag: &'static str, render: R) -> NodesExtensions<'a>
    where
        II: Iterator<Item = I>,
        for<'r> R: Fn(&I, crate::Svg<'r, C>),
    {
        self.list_with_render(items, ListElementCreation::Clone, tag, render)
    }

    fn list<I, II>(self, items: II, mode: ListElementCreation) -> NodesExtensions<'a>
    where
        I: SvgListItemRender<C>,
        II: Iterator<Item = I>,
    {
        self.list_with_render(items, mode, I::ROOT_ELEMENT_TAG, I::render)
    }

    fn list_clone<I, II>(self, items: II) -> NodesExtensions<'a>
    where
        I: SvgListItemRender<C>,
        II: Iterator<Item = I>,
    {
        self.list_with_render(
            items,
            ListElementCreation::Clone,
            I::ROOT_ELEMENT_TAG,
            I::render,
        )
    }
}

impl<'a, C: Component> MakeNodesExtensions<'a> for SvgElementRender<'a, C> {
    fn make_nodes_extensions(self) -> NodesExtensions<'a> {
        let e = self.into_inner().into_parts().3;
        NodesExtensions::new(e.nodes_mut())
    }
}

impl<'a, C: Component> MakeNodesExtensions<'a> for SvgAttributesOnly<'a, C> {
    fn make_nodes_extensions(self) -> NodesExtensions<'a> {
        let e = self.into_inner().into_parts().3;
        NodesExtensions::new(e.nodes_mut())
    }
}

impl<'a, C: Component> MakeNodesExtensions<'a> for SvgStaticAttributes<'a, C> {
    fn make_nodes_extensions(self) -> NodesExtensions<'a> {
        let e = self.into_inner().into_parts().3;
        NodesExtensions::new(e.nodes_mut())
    }
}

impl<'a, C: Component> MakeNodesExtensions<'a> for SvgStaticAttributesOnly<'a, C> {
    fn make_nodes_extensions(self) -> NodesExtensions<'a> {
        let e = self.into_inner().into_parts().3;
        NodesExtensions::new(e.nodes_mut())
    }
}

impl<'a, C: Component> SemsForList<'a, C> for SvgElementRender<'a, C> {}
impl<'a, C: Component> SemsForList<'a, C> for SvgAttributesOnly<'a, C> {}
impl<'a, C: Component> SemsForList<'a, C> for SvgStaticAttributes<'a, C> {}
impl<'a, C: Component> SemsForList<'a, C> for SvgStaticAttributesOnly<'a, C> {}
