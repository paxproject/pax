use pax_message::{AnyCreatePatch, ButtonPatch};
use pax_runtime::api::Layer;
use pax_runtime::declarative_macros::handle_vtable_update;
use pax_runtime::{
    BaseInstance, ExpandedNode, InstanceFlags, InstanceNode, InstantiationArgs, RuntimeContext,
};
use pax_std::primitives::Button;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::patch_if_needed;

pub struct ButtonInstance {
    base: BaseInstance,
    //Used as a cache of last-sent values, for crude dirty-checking.
    //Hopefully, this will by obviated by the built-in expression dirty-checking mechanism.
    last_patches: RefCell<HashMap<Vec<u32>, pax_message::ButtonPatch>>,
}

impl InstanceNode for ButtonInstance {
    fn instantiate(args: InstantiationArgs) -> Rc<Self>
    where
        Self: Sized,
    {
        Rc::new(Self {
            base: BaseInstance::new(
                args,
                InstanceFlags {
                    invisible_to_slot: false,
                    invisible_to_raycasting: false,
                    layer: Layer::Native,
                    is_component: false,
                },
            ),
            last_patches: Default::default(),
        })
    }

    fn update(self: Rc<Self>, expanded_node: &Rc<ExpandedNode>, context: &mut RuntimeContext) {
        expanded_node.with_properties_unwrapped(|properties: &mut Button| {
            let tbl = context.expression_table();
            let stk = &expanded_node.stack;
            handle_vtable_update(tbl, stk, &mut properties.label, context.globals());

            // Style
            handle_vtable_update(tbl, stk, &mut properties.style, context.globals());
            let stl = properties.style.get_mut();
            handle_vtable_update(tbl, stk, &mut stl.fill, context.globals());
            handle_vtable_update(tbl, stk, &mut stl.font, context.globals());
            handle_vtable_update(tbl, stk, &mut stl.font_size, context.globals());
            handle_vtable_update(tbl, stk, &mut stl.underline, context.globals());
            handle_vtable_update(tbl, stk, &mut stl.align_vertical, context.globals());
            handle_vtable_update(tbl, stk, &mut stl.align_horizontal, context.globals());
            handle_vtable_update(tbl, stk, &mut stl.align_multiline, context.globals());
        });
    }

    fn handle_native_patches(&self, expanded_node: &ExpandedNode, context: &mut RuntimeContext) {
        let id_chain = expanded_node.id_chain.clone();
        let mut patch = ButtonPatch {
            id_chain: id_chain.clone(),
            ..Default::default()
        };
        let mut last_patches = self.last_patches.borrow_mut();
        let old_state = last_patches.entry(id_chain.clone()).or_insert(ButtonPatch {
            id_chain,
            ..Default::default()
        });

        expanded_node.with_properties_unwrapped(|properties: &mut Button| {
            let layout_properties = expanded_node.layout_properties.borrow();
            let computed_tab = &layout_properties.as_ref().unwrap().computed_tab;
            let updates = [
                patch_if_needed(
                    &mut old_state.content,
                    &mut patch.content,
                    properties.label.get().string.clone(),
                ),
                patch_if_needed(
                    &mut old_state.style,
                    &mut patch.style,
                    properties.style.get().into(),
                ),
                patch_if_needed(
                    &mut old_state.size_x,
                    &mut patch.size_x,
                    computed_tab.bounds.0,
                ),
                patch_if_needed(
                    &mut old_state.size_y,
                    &mut patch.size_y,
                    computed_tab.bounds.1,
                ),
                patch_if_needed(
                    &mut old_state.transform,
                    &mut patch.transform,
                    computed_tab.transform.coeffs().to_vec(),
                ),
            ];
            if updates.into_iter().any(|v| v == true) {
                context.enqueue_native_message(pax_message::NativeMessage::ButtonUpdate(patch));
            }
        });
    }

    fn handle_mount(&self, expanded_node: &Rc<ExpandedNode>, context: &mut RuntimeContext) {
        context.enqueue_native_message(pax_message::NativeMessage::ButtonCreate(AnyCreatePatch {
            id_chain: expanded_node.id_chain.clone(),
            clipping_ids: vec![],
            scroller_ids: vec![],
            z_index: 0,
        }));
    }

    fn handle_unmount(&self, expanded_node: &Rc<ExpandedNode>, context: &mut RuntimeContext) {
        let id_chain = expanded_node.id_chain.clone();
        context.enqueue_native_message(pax_message::NativeMessage::ButtonDelete(id_chain));
    }

    fn base(&self) -> &BaseInstance {
        &self.base
    }

    #[cfg(debug_assertions)]
    fn resolve_debug(
        &self,
        f: &mut std::fmt::Formatter,
        _expanded_node: Option<&ExpandedNode>,
    ) -> std::fmt::Result {
        f.debug_struct("Button").finish_non_exhaustive()
    }
}
