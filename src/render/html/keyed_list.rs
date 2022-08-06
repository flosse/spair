use crate::component::Component;
use crate::dom::{Key, Keyed, NameSpace};
use crate::render::base::{ElementRender, ElementRenderMut, MakeNodesExtensions, NodesExtensions};
use crate::render::html::{
    AttributesOnly, HtmlElementRender, HtmlNameSpace, StaticAttributes, StaticAttributesOnly,
};
use crate::render::ListElementCreation;

pub trait HemsForKeyedList<'a, C: Component>:
    Sized + ElementRenderMut<C> + MakeNodesExtensions<'a>
{
    fn keyed_list_with_render<I, II, G, K, R>(
        mut self,
        items: II,
        mode: ListElementCreation,
        tag: &'a str,
        fn_get_key: G,
        fn_render: R,
    ) -> NodesExtensions<'a>
    where
        I: Copy,
        II: IntoIterator<Item = I>,
        G: Fn(I) -> K,
        K: Into<Key> + PartialEq<Key>,
        for<'u> R: Fn(I, HtmlElementRender<'u, C>),
    {
        let fn_render = |item: I, element: ElementRender<C>| {
            fn_render(item, element.into());
        };
        let _select_element_value_will_be_set_on_dropping_of_the_manager =
            self.element_render_mut().keyed_list_with_render(
                items,
                mode,
                tag,
                HtmlNameSpace::NAMESPACE,
                fn_get_key,
                fn_render,
            );
        self.make_nodes_extensions()
    }

    fn keyed_list<I, II>(self, items: II, mode: ListElementCreation) -> NodesExtensions<'a>
    where
        for<'k> I: Copy + Keyed<'k> + super::ListItemRender<C>,
        II: IntoIterator<Item = I>,
    {
        self.keyed_list_with_render(items, mode, I::ROOT_ELEMENT_TAG, I::key, I::render)
    }

    fn klwr_clone<I, II, G, K, R>(
        self,
        items: II,
        tag: &'a str,
        fn_get_key: G,
        fn_render: R,
    ) -> NodesExtensions<'a>
    where
        I: Copy,
        II: IntoIterator<Item = I>,
        G: Fn(I) -> K,
        K: Into<Key> + PartialEq<Key>,
        for<'u> R: Fn(I, HtmlElementRender<'u, C>),
    {
        self.keyed_list_with_render(
            items,
            ListElementCreation::Clone,
            tag,
            fn_get_key,
            fn_render,
        )
    }
}

impl<'a, C: Component> HemsForKeyedList<'a, C> for HtmlElementRender<'a, C> {}
impl<'a, C: Component> HemsForKeyedList<'a, C> for AttributesOnly<'a, C> {}
impl<'a, C: Component> HemsForKeyedList<'a, C> for StaticAttributes<'a, C> {}
impl<'a, C: Component> HemsForKeyedList<'a, C> for StaticAttributesOnly<'a, C> {}