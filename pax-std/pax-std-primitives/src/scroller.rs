use core::option::Option;
use core::option::Option::Some;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use kurbo::BezPath;
use piet::RenderContext;

use pax_runtime::{
    recurse_expand_nodes, ExpandedNode, HandlerRegistry, InstanceNode, InstanceNodePtr,
    InstanceNodePtrList, InstantiationArgs, PropertiesComputable, PropertiesTreeContext,
    RenderTreeContext,
};
use pax_message::{AnyCreatePatch, ScrollerPatch};
use pax_runtime::api::{
    ArgsScroll, CommonProperties, EasingCurve, Layer, PropertyInstance, PropertyLiteral, Size,
};
use pax_std::primitives::Scroller;

/// A combination of a clipping area (nearly identical to a `Frame`,) and an
/// inner panel that can be scrolled on zero or more axes.  `Scroller` coordinates with each chassis to
/// create native scrolling containers, which pass native scroll events back to Engine.  In turn,
/// `Scroller` translates its children to reflect the current scroll position.
/// When both scrolling axes are disabled, `Scroller` acts exactly like a `Frame`, with a possibly-
/// transformed `Group` surrounding its contents.
pub struct ScrollerInstance<R: 'static + RenderContext> {
    base: BaseInstance,
    pub scroll_x: f64,
    pub scroll_y: f64,
    pub scroll_x_offset: Rc<RefCell<dyn PropertyInstance<f64>>>,
    pub scroll_y_offset: Rc<RefCell<dyn PropertyInstance<f64>>>,
    last_patches: HashMap<Vec<u32>, ScrollerPatch>,
}

impl<R: 'static + RenderContext> InstanceNode<R> for ScrollerInstance<R> {
    fn get_layer_type(&mut self) -> Layer {
        Layer::Scroller
    }

    fn new(args: InstantiationArgs<R>) -> Rc<RefCell<Self>>
    where
        Self: Sized,
    {
        Rc::new(RefCell::new(Self {
            last_patches: HashMap::new(),
            scroll_x: 0.0,
            scroll_y: 0.0,
            scroll_x_offset: Rc::new(RefCell::new(PropertyLiteral::new(0.0))),
            scroll_y_offset: Rc::new(RefCell::new(PropertyLiteral::new(0.0))),
            base: BaseInstance::new(args),
        }))
    }

    fn handle_scroll(&mut self, args_scroll: ArgsScroll) {
        self.scroll_x -= args_scroll.delta_x;
        self.scroll_y -= args_scroll.delta_y;
        (*self.scroll_x_offset)
            .borrow_mut()
            .ease_to(self.scroll_x, 2, EasingCurve::Linear);
        (*self.scroll_y_offset)
            .borrow_mut()
            .ease_to(self.scroll_y, 2, EasingCurve::Linear);
    }

    fn get_scroll_offset(&mut self) -> (f64, f64) {
        (
            (*self.scroll_x_offset).borrow().get().clone(),
            (*self.scroll_y_offset).borrow().get().clone(),
        )
    }

    fn get_handler_registry(&self) -> Option<Rc<RefCell<HandlerRegistry<R>>>> {
        match &self.handler_registry {
            Some(registry) => Some(Rc::clone(&registry)),
            _ => None,
        }
    }

    fn handle_native_patches(
        &mut self,
        ptc: &mut PropertiesTreeContext<R>,
        computed_size: (f64, f64),
        transform_coeffs: Vec<f64>,
        _z_index: u32,
        subtree_depth: u32,
    ) {
        // let mut new_message: ScrollerPatch = Default::default();
        // new_message.id_chain = ptc.get_id_chain();
        // if !self.last_patches.contains_key(&new_message.id_chain) {
        //     let mut patch = ScrollerPatch::default();
        //     patch.id_chain = new_message.id_chain.clone();
        //     self.last_patches
        //         .insert(new_message.id_chain.clone(), patch);
        // }
        // let last_patch = self.last_patches.get_mut(&new_message.id_chain).unwrap();
        // let mut has_any_updates = false;
        //
        // let wrapped = ptc.current_expanded_node.as_ref().unwrap().clone().borrow().get_properties().clone();
        // with_properties_unsafe!(&wrapped, PropertiesCoproduct, Scroller, |properties: &mut Scroller| {
        //     let val = subtree_depth;
        //     let is_new_value = last_patch.subtree_depth != val;
        //     if is_new_value {
        //         new_message.subtree_depth = val;
        //         last_patch.subtree_depth = val;
        //         has_any_updates = true;
        //     }
        //
        // let val = computed_size.0;
        // let is_new_value = match &last_patch.size_x {
        //     Some(cached_value) => !val.eq(cached_value),
        //     None => true,
        // };
        // if is_new_value {
        //     new_message.size_x = Some(val);
        //     last_patch.size_x = Some(val);
        //     has_any_updates = true;
        // }
        //
        // let val = computed_size.1;
        // let is_new_value = match &last_patch.size_y {
        //     Some(cached_value) => !val.eq(cached_value),
        //     None => true,
        // };
        // if is_new_value {
        //     new_message.size_y = Some(val);
        //     last_patch.size_y = Some(val);
        //     has_any_updates = true;
        // }
        //
        // let val = Size::get_pixels(properties.size_inner_pane_x.get(), computed_size.0);
        // let is_new_value = match &last_patch.size_inner_pane_x {
        //     Some(cached_value) => !val.eq(cached_value),
        //     None => true,
        // };
        // if is_new_value {
        //     new_message.size_inner_pane_x = Some(val);
        //     last_patch.size_inner_pane_x = Some(val);
        //     has_any_updates = true;
        // }
        //
        // let val = Size::get_pixels(properties.size_inner_pane_y.get(), computed_size.1);
        // let is_new_value = match &last_patch.size_inner_pane_y {
        //     Some(cached_value) => !val.eq(cached_value),
        //     None => true,
        // };
        // if is_new_value {
        //     new_message.size_inner_pane_y = Some(val);
        //     last_patch.size_inner_pane_y = Some(val);
        //     has_any_updates = true;
        // }
        //
        // let val = properties.scroll_enabled_x.get();
        // let is_new_value = match &last_patch.scroll_x {
        //     Some(cached_value) => !val.eq(cached_value),
        //     None => true,
        // };
        // if is_new_value {
        //     new_message.scroll_x = Some(*val);
        //     last_patch.scroll_x = Some(*val);
        //     has_any_updates = true;
        // }
        //
        // let val = properties.scroll_enabled_y.get();
        // let is_new_value = match &last_patch.scroll_y {
        //     Some(cached_value) => !val.eq(cached_value),
        //     None => true,
        // };
        // if is_new_value {
        //     new_message.scroll_y = Some(*val);
        //     last_patch.scroll_y = Some(*val);
        //     has_any_updates = true;
        // }
        //
        //     let latest_transform = transform_coeffs;
        //     let is_new_transform = match &last_patch.transform {
        //         Some(cached_transform) => latest_transform
        //             .iter()
        //             .enumerate()
        //             .any(|(i, elem)| *elem != cached_transform[i]),
        //         None => true,
        //     };
        //     if is_new_transform {
        //         new_message.transform = Some(latest_transform.clone());
        //         last_patch.transform = Some(latest_transform.clone());
        //         has_any_updates = true;
        //     }
        //
        //     if has_any_updates {
        //         ptc.enqueue_native_message(pax_message::NativeMessage::ScrollerUpdate(new_message));
        //     }
        // });
        todo!()
    }

    fn get_instance_children(&self) -> InstanceNodePtrList<R> {
        Rc::clone(&self.children)
    }

    fn get_clipping_size(&self, expanded_node: &ExpandedNode<R>) -> Option<(Size, Size)> {
        let comm_props = expanded_node.get_common_properties();

        // `ret` temp appeases borrow checker
        let ret = Some((
            comm_props.borrow().width.get().clone(),
            comm_props.borrow().height.get().clone(),
        ));
        ret
    }

    /// Scroller's `size` is the size of its inner pane
    fn get_size(&self, expanded_node: &ExpandedNode<R>) -> (Size, Size) {
        let properties_wrapped = expanded_node.get_properties();
        // with_properties_unsafe!(properties_wrapped, PropertiesCoproduct, Scroller, |properties : &mut Scroller|{
        //     (
        //         properties
        //             .size_inner_pane_x
        //             .get()
        //             .clone(),
        //         properties
        //             .size_inner_pane_y
        //             .get()
        //             .clone(),
        //     )
        // })
        todo!()
    }

    fn expand_node_and_compute_properties(
        &mut self,
        ptc: &mut PropertiesTreeContext<R>,
    ) -> Rc<RefCell<ExpandedNode<R>>> {
        let id_chain = ptc.get_id_chain();

        // if true {
        todo!("manage vtable evaluation for own properties");
        // }

        ptc.push_clipping_stack_id(id_chain.clone());
        ptc.push_scroller_stack_id(id_chain.clone());

        for child in self.get_instance_children().borrow().iter() {
            let new_ptc = ptc.clone();
            // let new_instance_node = Rc::clone(child);
            // let new_expanded_node = new_instance_node.borrow().com
            //
            // recurse_expand_nodes();
            todo!("manage children")
        }

        ptc.pop_clipping_stack_id();
        ptc.pop_scroller_stack_id();
        // self.common_properties.compute_properties(ptc);
        //
        // let mut scroll_x_offset_borrowed = (*self.scroll_x_offset).borrow_mut();
        // if let Some(new_value) =
        //     ptc.compute_eased_value(scroll_x_offset_borrowed._get_transition_manager())
        // {
        //     scroll_x_offset_borrowed.set(new_value);
        // }
        //
        // let mut scroll_y_offset_borrowed = (*self.scroll_y_offset).borrow_mut();
        // if let Some(new_value) =
        //     ptc.compute_eased_value(scroll_y_offset_borrowed._get_transition_manager())
        // {
        //     scroll_y_offset_borrowed.set(new_value);
        // }
        //
        // let properties = &mut *self.properties.as_ref().borrow_mut();
        //
        // if let Some(new_size) =
        //     ptc.compute_vtable_value(properties.size_inner_pane_x._get_vtable_id())
        // {
        //     let new_value = if let TypesCoproduct::Size(v) = new_size {
        //         v
        //     } else {
        //         unreachable!()
        //     };
        //     properties.size_inner_pane_x.set(new_value);
        // }
        //
        // if let Some(new_size) =
        //     ptc.compute_vtable_value(properties.size_inner_pane_y._get_vtable_id())
        // {
        //     let new_value = if let TypesCoproduct::Size(v) = new_size {
        //         v
        //     } else {
        //         unreachable!()
        //     };
        //     properties.size_inner_pane_y.set(new_value);
        // }
        //
        // if let Some(scroll_enabled_x) =
        //     ptc.compute_vtable_value(properties.scroll_enabled_x._get_vtable_id())
        // {
        //     let new_value = if let TypesCoproduct::bool(v) = scroll_enabled_x {
        //         v
        //     } else {
        //         unreachable!()
        //     };
        //     properties.scroll_enabled_x.set(new_value);
        // }
        //
        // if let Some(scroll_enabled_y) =
        //     ptc.compute_vtable_value(properties.scroll_enabled_y._get_vtable_id())
        // {
        //     let new_value = if let TypesCoproduct::bool(v) = scroll_enabled_y {
        //         v
        //     } else {
        //         unreachable!()
        //     };
        //     properties.scroll_enabled_y.set(new_value);
        // }
        //
        // self.common_properties.compute_properties(ptc);
    }

    fn manages_own_subtree_for_expansion(&self) -> bool {
        true
    }

    fn handle_pre_render(&mut self, rtc: &mut RenderTreeContext<R>, rcs: &mut HashMap<String, R>) {
        let expanded_node = rtc.current_expanded_node.borrow();
        let tab = &expanded_node.tab;

        let width: f64 = tab.bounds.0;
        let height: f64 = tab.bounds.1;

        let mut bez_path = BezPath::new();
        bez_path.move_to((0.0, 0.0));
        bez_path.line_to((width, 0.0));
        bez_path.line_to((width, height));
        bez_path.line_to((0.0, height));
        bez_path.line_to((0.0, 0.0));
        bez_path.close_path();

        let transformed_bez_path = tab.transform * bez_path;
        for (_key, rc) in rcs.iter_mut() {
            rc.save().unwrap(); //our "save point" before clipping — restored to in the post_render
            rc.clip(transformed_bez_path.clone());
        }
        let id_chain = rtc.current_expanded_node.borrow().id_chain.clone();
    }
    fn handle_post_render(
        &mut self,
        rtc: &mut RenderTreeContext<R>,
        _rcs: &mut HashMap<String, R>,
    ) {
        for (_key, rc) in _rcs.iter_mut() {
            //pop the clipping context from the stack
            rc.restore().unwrap();
        }
    }

    fn handle_mount(&mut self, ptc: &mut PropertiesTreeContext<R>) {
        let id_chain = ptc.get_id_chain();
        let z_index = ptc.current_expanded_node.as_ref().unwrap().borrow().z_index;

        //though macOS and iOS don't need this ancestry chain for clipping, Web does
        let clipping_ids = ptc.get_current_clipping_ids();

        let scroller_ids = ptc.get_current_scroller_ids();

        ptc.enqueue_native_message(pax_message::NativeMessage::ScrollerCreate(AnyCreatePatch {
            id_chain: id_chain.clone(),
            clipping_ids,
            scroller_ids,
            z_index,
        }));
    }

    fn handle_unmount(&mut self, ptc: &mut PropertiesTreeContext<R>) {
        let id_chain = ptc.get_id_chain();
        self.last_patches.remove(&id_chain);
        ptc.enqueue_native_message(pax_message::NativeMessage::ScrollerDelete(id_chain));
    }

    fn base(&self) -> &BaseInstance {
        &self.base
    }
}
