use super::ElementRender;
use crate::component::{Comp, Component};
use crate::dom::Nodes;

#[must_use = "Caller should set selected option for <select> element"]
pub struct RememberSettingSelectedOption;

pub struct ListRender<'a, C: Component> {
    pub(crate) comp: &'a Comp<C>,
    pub(crate) state: &'a C,
    pub(crate) tag: &'a str,
    pub(crate) name_space: Option<&'a str>,
    pub(crate) use_template: bool,
    pub(crate) parent: &'a web_sys::Node,
    // This is None if the list is the only content of the parent node.
    // This is Some(thing) if the list is just a part of the parent node.
    // In other words, a part from the list, the parent also contains other
    // nodes before or/and after the list's nodes.
    pub(crate) end_of_list_flag: Option<&'a web_sys::Node>,
    pub(crate) list: &'a mut Nodes,
}

impl<'a, C: Component> ListRender<'a, C> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        comp: &'a Comp<C>,
        state: &'a C,
        list: &'a mut Nodes,
        tag: &'a str,
        name_space: Option<&'a str>,
        parent: &'a web_sys::Node,
        end_of_list_flag: Option<&'a web_sys::Node>,
        use_template: bool,
    ) -> Self {
        Self {
            comp,
            state,
            tag,
            name_space,
            use_template,
            parent,
            end_of_list_flag,
            list,
        }
    }

    pub fn clear_after(&mut self, index: usize) {
        if index >= self.list.count() {
            return;
        }
        if index == 0 && self.end_of_list_flag.is_none() {
            self.parent.set_text_content(None);
            self.list.clear_vec();
        } else {
            self.list.clear_after(index, self.parent);
        }
    }

    pub fn render<I, R>(
        &mut self,
        items: impl IntoIterator<Item = I>,
        render: R,
    ) -> RememberSettingSelectedOption
    where
        I: Copy,
        for<'u> R: Fn(I, ElementRender<'u, C>),
    {
        let mut index = 0;
        for item in items {
            let status = self.list.check_or_create_element_for_list(
                self.tag,
                self.name_space,
                index,
                self.parent,
                self.end_of_list_flag,
                self.use_template,
            );
            let element = self.list.get_element_mut(index);
            let u = ElementRender::new(self.comp, self.state, element, status);
            render(item, u);
            index += 1;
        }
        self.clear_after(index);
        RememberSettingSelectedOption
    }
}
