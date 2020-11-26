use wasm_bindgen::{JsCast, UnwrapThrowExt};

pub struct StaticAttributes<'a, C: crate::component::Component>(crate::dom::ElementUpdater<'a, C>);

impl<'a, C: crate::component::Component> StaticAttributes<'a, C> {
    pub(in crate::dom) fn new(eu: crate::dom::ElementUpdater<'a, C>) -> Self {
        Self(eu)
    }

    pub fn nodes(self) -> crate::dom::NodesOwned<'a, C> {
        self.0.nodes()
    }

    pub fn static_nodes(self) -> crate::dom::StaticNodesOwned<'a, C> {
        self.0.static_nodes()
    }

    /// Use this method when the compiler complains about expected `()` but found something else and you don't want to add a `;`
    pub fn done(self) {}

    pub fn render(self, value: impl crate::renderable::Render<C>) -> crate::dom::NodesOwned<'a, C> {
        self.0.render(value)
    }

    pub fn render_ref(
        self,
        value: &impl crate::renderable::RenderRef<C>,
    ) -> crate::dom::NodesOwned<'a, C> {
        self.0.render_ref(value)
    }

    pub fn r#static(
        self,
        value: impl crate::renderable::StaticRender<C>,
    ) -> crate::dom::NodesOwned<'a, C> {
        self.0.r#static(value)
    }

    pub fn list<I>(self, items: impl IntoIterator<Item = I>, mode: crate::dom::ListElementCreation)
    where
        I: crate::renderable::ListItem<C>,
    {
        self.0.list(items, mode)
    }

    pub fn list_with_render<I, R>(
        self,
        items: impl IntoIterator<Item = I>,
        mode: crate::dom::ListElementCreation,
        tag: &str,
        render: R,
    ) where
        for<'i, 'c> R: Fn(&'i I, crate::Element<'c, C>),
    {
        self.0.list_with_render(items, mode, tag, render)
    }

    #[cfg(feature = "keyed-list")]
    pub fn keyed_list<I>(
        self,
        items: impl IntoIterator<Item = I>,
        mode: crate::dom::ListElementCreation,
    ) where
        for<'k> I: crate::dom::KeyedListItem<'k, C>,
    {
        self.0.keyed_list(items, mode)
    }

    pub fn component<CC: crate::component::Component>(
        self,
        child: &crate::component::ChildComp<CC>,
    ) {
        self.0.component(child);
    }
}

pub trait AttributeSetter<C>: Sized + crate::dom::attributes::AttributeSetter
where
    C: crate::component::Component,
{
    create_methods_for_events! {
        on_focus Focus,
        on_blur Blur,

        on_aux_click AuxClick,
        on_click Click,
        on_double_click DoubleClick,
        on_mouse_enter MouseEnter,
        on_mouse_over MouseOver,
        on_mouse_move MouseMove,
        on_mouse_down MouseDown,
        on_mouse_up MouseUp,
        on_mouse_leave MouseLeave,
        on_mouse_out MouseOut,
        on_context_menu ContextMenu,

        on_wheel Wheel,
        on_select UiSelect,

        on_input Input,

        on_key_down KeyDown,
        on_key_press KeyPress,
        on_key_up KeyUp,

        on_change Change,
        on_reset Reset,
        on_submit Submit,
        on_pointer_lock_change PointerLockChange,
        on_pointer_lock_error PointerLockError,

        on_ended Ended,
    }

    fn bool_attr(mut self, name: &str, value: bool) -> Self {
        self.set_bool_attribute(name, value);
        self
    }

    fn str_attr(mut self, name: &str, value: &str) -> Self {
        self.set_str_attribute(name, value);
        self
    }

    fn i32_attr(mut self, name: &str, value: i32) -> Self {
        self.set_i32_attribute(name, value);
        self
    }

    fn u32_attr(mut self, name: &str, value: u32) -> Self {
        self.set_u32_attribute(name, value);
        self
    }

    fn f64_attr(mut self, name: &str, value: f64) -> Self {
        self.set_f64_attribute(name, value);
        self
    }

    create_methods_for_attributes! {
        str     abbr
        str     accept
        str     accept_charset "accept-charset"
        str     action
        str     allow
        str     allow_full_screen "allowfullscreen"
        bool    allow_payment_request "allowpaymentrequest"
        str     alt
        AsStr   auto_complete "autocomplete"
        bool    auto_play "autoplay"
        str     cite
        //str     class
        u32     cols
        u32     col_span "colspan"
        bool    controls
        str     coords
        AsStr   cross_origin "crossorigin"
        str     data
        str     date_time "datetime"
        AsStr   decoding
        bool    default
        str     dir_name "dirname"
        bool    disabled
        str     download
        AsStr   enc_type "enctype"
        str     r#for "for"
        str     form
        str     form_action "formaction"
        AsStr   form_enc_type "formenctype"
        AsStr   form_method "formmethod"
        bool    form_no_validate "formnovalidate"
        AsStr   form_target "formtarget"
        str     headers
        u32     height
        bool    hidden
        f64     high
        str     href_str "href" // method named `href` is used for routing
        str     href_lang "hreflang"
        bool    is_map "ismap"
        AsStr   kind
        str     label
        bool    r#loop "loop"
        f64     low
        // ??   max: what type? split into multiple methods?
        i32     max_length "maxlength"
        str     media
        AsStr   method
        // ??   min: similar to max
        i32     min_length "minlength"
        bool    multiple
        bool    muted
        str     name
        bool    no_validate "novalidate"
        bool    open
        f64     optimum
        str     pattern
        str     ping
        str     placeholder
        str     poster
        bool    plays_inline "playsinline"
        AsStr   pre_load "preload"
        bool    read_only "readonly"
        AsStr   referrer_policy "referrerpolicy"
        str     rel
        // ??     rellist
        bool    required
        bool    reversed
        u32     rows
        u32     row_span "rowspan"
        // ?? sandbox
        bool    selected
        AsStr   scope
        u32     size
        str     sizes
        u32     span_attr "span" // rename to `span_attr` to avoid conflict with DomBuilder::span
        str     src
        str     src_doc "srcdoc"
        str     src_lang "srclang"
        str     src_set "srcset"
        i32     start
        str     step
        str     style
        AsStr   target
        str     title
        AsStr   r#type "type"
        str     use_map "usemap"
        u32     width
        AsStr   wrap
    }

    /// Only execute `input.set_checked` if the value changed.
    fn checked_if_changed(mut self, value: bool) -> Self {
        if self.check_bool_attribute(value) {
            self.checked(value)
        } else {
            self
        }
    }

    /// Always execute `input.set_checked` with the given value. This is
    /// useful in situation like in TodoMVC example. TodoMVC spec requires
    /// that when the app in a filtered mode, for example, just display
    /// active todos, if an item is checked (completed) by clicking the
    /// input, the app should hide the todo item. In such a situation, the
    /// DOM item is checked, but Spair DOM is not checked yet. But the
    /// checked item was filtered out (hidden), and only active todos
    /// are displayed, all of them are unchecked which match the state in
    /// Spair DOM, hence Spair skip setting check, leaving the DOM checked
    /// but display an unchecked item. In my understand, this only occurs
    /// with non-keyed list. I choose always setting checked to avoid
    /// surprise for new users. `checked_if_changed` can be used to reduce
    /// interaction with DOM if it does not bug you.
    fn checked(self, value: bool) -> Self {
        let element = self.ws_element();
        if self.element_type() == crate::dom::ElementType::Input {
            let input = element.unchecked_ref::<web_sys::HtmlInputElement>();
            input.set_checked(value);
        } else {
            log::warn!(".checked() is called on an element that is not <input>");
        }
        self
    }

    /// This method should only execute in static mode.
    fn class(self, class_name: &str) -> Self {
        self.ws_element()
            .class_list()
            .add_1(class_name)
            .expect_throw("Unable to add class");
        self
    }

    fn class_if(mut self, class_name: &str, class_on: bool) -> Self {
        if self.check_bool_attribute(class_on) {
            if class_on {
                self.ws_element()
                    .class_list()
                    .add_1(class_name)
                    .expect_throw("Unable to add class");
            } else {
                self.ws_element()
                    .class_list()
                    .remove_1(class_name)
                    .expect_throw("Unable to remove class");
            }
        }
        self
    }

    fn enabled(self, value: bool) -> Self {
        self.disabled(!value)
    }

    fn focus(mut self, value: bool) -> Self {
        if value && self.check_bool_attribute(value) {
            self.ws_html_element()
                .focus()
                .expect_throw("Unable to set focus");
        }
        self
    }

    fn href(mut self, value: &C::Routes) -> Self {
        use crate::routing::Routes;
        let url = value.url();
        if self.check_str_attribute(&url) {
            self.set_str_attribute("href", &url);
        }
        self
    }

    fn id(mut self, id: &str) -> Self {
        if self.check_str_attribute(id) {
            self.ws_element().set_id(id);
        }
        self
    }

    fn max(self, value: impl AttributeMax<Self>) -> Self {
        value.update(self)
    }

    fn min(self, value: impl AttributeMin<Self>) -> Self {
        value.update(self)
    }

    fn selected_value(mut self, value: Option<&str>) -> Self {
        if self.element_type() == crate::dom::ElementType::Select {
            // TODO: check to find change of value?

            // It has no effect if you set a value for
            // a <select> element before adding its <option>s,
            // the hacking should finish in the list() method.
            // Is there a better solution?
            self.set_selected_value(value);
        } else {
            log::warn!(".selected_value() can only be called on <select>");
        }
        self
    }

    fn selected_index(mut self, index: Option<usize>) -> Self {
        if self.element_type() == crate::dom::ElementType::Select {
            // TODO: check to find change of index?

            // It has no effect if you set a selected index for
            // a <select> element before adding its <option>s,
            // the hacking should finish in the list() method.
            // Is there a better solution?
            self.set_selected_index(index);
        } else {
            log::warn!(".selected_index() is called on an element that is not <select>");
        }
        self
    }

    fn value(self, value: impl AttributeValue<Self>) -> Self {
        value.update(self)
    }
}

impl<'a, C: crate::component::Component> AttributeSetter<C> for StaticAttributes<'a, C> where
    C: crate::component::Component
{
}

impl<'a, C: crate::component::Component> crate::dom::attributes::AttributeSetter
    for StaticAttributes<'a, C>
{
    fn ws_html_element(&self) -> &web_sys::HtmlElement {
        self.0.element.ws_element.unchecked_ref()
    }

    fn ws_element(&self) -> &web_sys::Element {
        &self.0.element.ws_element
    }

    fn element_type(&self) -> crate::dom::ElementType {
        self.0.element.element_type
    }

    fn require_set_listener(&mut self) -> bool {
        if self.0.status == crate::dom::ElementStatus::Existing {
            // When self.require_init == false, self.store_listener will not be invoked.
            // We must update the index here to count over the static events.
            self.0.index += 1;
            false
        } else {
            // A cloned element requires its event handlers to be set because the events
            // are not cloned.
            true
        }
    }

    fn store_listener(&mut self, listener: Box<dyn crate::events::Listener>) {
        self.0
            .element
            .attributes
            .store_listener(self.0.index, listener);
        self.0.index += 1;
    }

    fn check_bool_attribute(&mut self, _value: bool) -> bool {
        self.0.status == crate::dom::ElementStatus::JustCreated
        // no need to store the value for static attributes
    }

    fn check_str_attribute(&mut self, _value: &str) -> bool {
        self.0.status == crate::dom::ElementStatus::JustCreated
        // no need to store the value for static attributes
    }

    fn check_i32_attribute(&mut self, _value: i32) -> bool {
        self.0.status == crate::dom::ElementStatus::JustCreated
        // no need to store the value for static attributes
    }

    fn check_u32_attribute(&mut self, _value: u32) -> bool {
        self.0.status == crate::dom::ElementStatus::JustCreated
        // no need to store the value for static attributes
    }

    fn check_f64_attribute(&mut self, _value: f64) -> bool {
        self.0.status == crate::dom::ElementStatus::JustCreated
        // no need to store the value for static attributes
    }

    fn set_selected_value(&mut self, value: Option<&str>) {
        self.0.select_element_value.set_selected_value(value);
    }

    fn set_selected_index(&mut self, index: Option<usize>) {
        self.0.select_element_value.set_selected_index(index);
    }
}

impl<'a, C: crate::component::Component> AttributeSetter<C> for crate::dom::ElementUpdater<'a, C> where
    C: crate::component::Component
{
}

impl<'a, C: crate::component::Component> crate::dom::attributes::AttributeSetter
    for crate::dom::ElementUpdater<'a, C>
{
    fn ws_html_element(&self) -> &web_sys::HtmlElement {
        self.element.ws_element.unchecked_ref()
    }

    fn ws_element(&self) -> &web_sys::Element {
        &self.element.ws_element
    }

    fn element_type(&self) -> crate::dom::ElementType {
        self.element.element_type
    }

    fn require_set_listener(&mut self) -> bool {
        true
    }

    fn store_listener(&mut self, listener: Box<dyn crate::events::Listener>) {
        self.element.attributes.store_listener(self.index, listener);
        self.index += 1;
    }

    fn check_bool_attribute(&mut self, value: bool) -> bool {
        let rs = self
            .element
            .attributes
            .check_bool_attribute(self.index, value);
        self.index += 1;
        rs
    }

    fn check_str_attribute(&mut self, value: &str) -> bool {
        let rs = self
            .element
            .attributes
            .check_str_attribute(self.index, value);
        self.index += 1;
        rs
    }

    fn check_i32_attribute(&mut self, value: i32) -> bool {
        let rs = self
            .element
            .attributes
            .check_i32_attribute(self.index, value);
        self.index += 1;
        rs
    }

    fn check_u32_attribute(&mut self, value: u32) -> bool {
        let rs = self
            .element
            .attributes
            .check_u32_attribute(self.index, value);
        self.index += 1;
        rs
    }

    fn check_f64_attribute(&mut self, value: f64) -> bool {
        let rs = self
            .element
            .attributes
            .check_f64_attribute(self.index, value);
        self.index += 1;
        rs
    }

    fn set_selected_value(&mut self, value: Option<&str>) {
        self.select_element_value.set_selected_value(value);
    }

    fn set_selected_index(&mut self, index: Option<usize>) {
        self.select_element_value.set_selected_index(index);
    }
}

// TODO: Should all these (below) be produced by macros?

pub trait AttributeValue<U> {
    fn update(self, u: U) -> U;
}

// &str
impl<'a, C: crate::component::Component> AttributeValue<crate::dom::ElementUpdater<'a, C>>
    for &str
{
    fn update(self, mut u: crate::dom::ElementUpdater<'a, C>) -> crate::dom::ElementUpdater<'a, C> {
        use crate::dom::attributes::AttributeSetter;
        u.value_str(self);
        u
    }
}

impl<'a, C: crate::component::Component> AttributeValue<crate::dom::StaticAttributes<'a, C>>
    for &str
{
    fn update(
        self,
        mut u: crate::dom::StaticAttributes<'a, C>,
    ) -> crate::dom::StaticAttributes<'a, C> {
        use crate::dom::attributes::AttributeSetter;
        u.value_str(self);
        u
    }
}

// &String
impl<'a, C: crate::component::Component> AttributeValue<crate::dom::ElementUpdater<'a, C>>
    for &String
{
    fn update(self, mut u: crate::dom::ElementUpdater<'a, C>) -> crate::dom::ElementUpdater<'a, C> {
        use crate::dom::attributes::AttributeSetter;
        u.value_str(self);
        u
    }
}

impl<'a, C: crate::component::Component> AttributeValue<crate::dom::StaticAttributes<'a, C>>
    for &String
{
    fn update(
        self,
        mut u: crate::dom::StaticAttributes<'a, C>,
    ) -> crate::dom::StaticAttributes<'a, C> {
        use crate::dom::attributes::AttributeSetter;
        u.value_str(self);
        u
    }
}

// Option<&str>
impl<'a, C: crate::component::Component> AttributeValue<crate::dom::ElementUpdater<'a, C>>
    for Option<&str>
{
    fn update(self, u: crate::dom::ElementUpdater<'a, C>) -> crate::dom::ElementUpdater<'a, C> {
        u.selected_value(self)
    }
}

impl<'a, C: crate::component::Component> AttributeValue<crate::dom::StaticAttributes<'a, C>>
    for Option<&str>
{
    fn update(self, u: crate::dom::StaticAttributes<'a, C>) -> crate::dom::StaticAttributes<'a, C> {
        u.selected_value(self)
    }
}

// f64
impl<'a, C: crate::component::Component> AttributeValue<crate::dom::ElementUpdater<'a, C>> for f64 {
    fn update(self, u: crate::dom::ElementUpdater<'a, C>) -> crate::dom::ElementUpdater<'a, C> {
        u.f64_attr("value", self)
    }
}

impl<'a, C: crate::component::Component> AttributeValue<crate::dom::StaticAttributes<'a, C>>
    for f64
{
    fn update(self, u: crate::dom::StaticAttributes<'a, C>) -> crate::dom::StaticAttributes<'a, C> {
        u.f64_attr("value", self)
    }
}

pub trait AttributeMax<U> {
    fn update(self, u: U) -> U;
}

// &str
impl<'a, C: crate::component::Component> AttributeMax<crate::dom::ElementUpdater<'a, C>> for &str {
    fn update(self, u: crate::dom::ElementUpdater<'a, C>) -> crate::dom::ElementUpdater<'a, C> {
        u.str_attr("max", self)
    }
}

impl<'a, C: crate::component::Component> AttributeMax<crate::dom::StaticAttributes<'a, C>>
    for &str
{
    fn update(self, u: crate::dom::StaticAttributes<'a, C>) -> crate::dom::StaticAttributes<'a, C> {
        u.str_attr("max", self)
    }
}

// &String
impl<'a, C: crate::component::Component> AttributeMax<crate::dom::ElementUpdater<'a, C>>
    for &String
{
    fn update(self, u: crate::dom::ElementUpdater<'a, C>) -> crate::dom::ElementUpdater<'a, C> {
        u.str_attr("max", self)
    }
}

impl<'a, C: crate::component::Component> AttributeMax<crate::dom::StaticAttributes<'a, C>>
    for &String
{
    fn update(self, u: crate::dom::StaticAttributes<'a, C>) -> crate::dom::StaticAttributes<'a, C> {
        u.str_attr("max", self)
    }
}

// f64
impl<'a, C: crate::component::Component> AttributeMax<crate::dom::ElementUpdater<'a, C>> for f64 {
    fn update(self, u: crate::dom::ElementUpdater<'a, C>) -> crate::dom::ElementUpdater<'a, C> {
        u.f64_attr("max", self)
    }
}

impl<'a, C: crate::component::Component> AttributeMax<crate::dom::StaticAttributes<'a, C>> for f64 {
    fn update(self, u: crate::dom::StaticAttributes<'a, C>) -> crate::dom::StaticAttributes<'a, C> {
        u.f64_attr("max", self)
    }
}

pub trait AttributeMin<U> {
    fn update(self, u: U) -> U;
}

// &str
impl<'a, C: crate::component::Component> AttributeMin<crate::dom::ElementUpdater<'a, C>> for &str {
    fn update(self, u: crate::dom::ElementUpdater<'a, C>) -> crate::dom::ElementUpdater<'a, C> {
        u.str_attr("min", self)
    }
}

impl<'a, C: crate::component::Component> AttributeMin<crate::dom::StaticAttributes<'a, C>>
    for &str
{
    fn update(self, u: crate::dom::StaticAttributes<'a, C>) -> crate::dom::StaticAttributes<'a, C> {
        u.str_attr("min", self)
    }
}

// &String
impl<'a, C: crate::component::Component> AttributeMin<crate::dom::ElementUpdater<'a, C>>
    for &String
{
    fn update(self, u: crate::dom::ElementUpdater<'a, C>) -> crate::dom::ElementUpdater<'a, C> {
        u.str_attr("min", self)
    }
}

impl<'a, C: crate::component::Component> AttributeMin<crate::dom::StaticAttributes<'a, C>>
    for &String
{
    fn update(self, u: crate::dom::StaticAttributes<'a, C>) -> crate::dom::StaticAttributes<'a, C> {
        u.str_attr("min", self)
    }
}

// f64
impl<'a, C: crate::component::Component> AttributeMin<crate::dom::ElementUpdater<'a, C>> for f64 {
    fn update(self, u: crate::dom::ElementUpdater<'a, C>) -> crate::dom::ElementUpdater<'a, C> {
        u.f64_attr("min", self)
    }
}

impl<'a, C: crate::component::Component> AttributeMin<crate::dom::StaticAttributes<'a, C>> for f64 {
    fn update(self, u: crate::dom::StaticAttributes<'a, C>) -> crate::dom::StaticAttributes<'a, C> {
        u.f64_attr("min", self)
    }
}