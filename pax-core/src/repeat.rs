use std::cell::RefCell;
use std::rc::Rc;

use crate::{ComponentInstance, InstantiationArgs, InstanceNode, InstanceNodePtr, InstanceNodePtrList, RenderTreeContext, ExpandedNode, PropertiesTreeContext, with_properties_unsafe, unsafe_wrap, unsafe_unwrap, handle_vtable_update_optional};
use pax_properties_coproduct::{PropertiesCoproduct, RepeatProperties, TypesCoproduct};
use pax_runtime_api::{CommonProperties, Layer, PropertyInstance, Size};
use piet_common::RenderContext;

/// A special "control-flow" primitive associated with the `for` statement.
/// Repeat allows for nodes to be rendered dynamically per data specified in `source_expression`.
/// That is: for a `source_expression` of length `n`, `Repeat` will render its
/// template `n` times, each with an embedded component context (`RepeatItem`)
/// with an index `i` and a pointer to that relevant datum `source_expression[i]`
pub struct RepeatInstance<R: 'static + RenderContext> {
    pub instance_id: u32,
    pub repeated_template: InstanceNodePtrList<R>,

    instance_prototypical_properties: Rc<RefCell<PropertiesCoproduct>>,
    instance_prototypical_common_properties: Rc<RefCell<CommonProperties>>,
}

impl<R: 'static + RenderContext> InstanceNode<R> for RepeatInstance<R> {
    fn get_instance_id(&self) -> u32 {
        self.instance_id
    }

    fn instantiate(args: InstantiationArgs<R>) -> Rc<RefCell<Self>>
    where
        Self: Sized,
    {
        let mut node_registry = (*args.node_registry).borrow_mut();
        let instance_id = node_registry.mint_instance_id();
        let ret = Rc::new(RefCell::new(RepeatInstance {
            instance_id,
            repeated_template: match args.children {
                None => Rc::new(RefCell::new(vec![])),
                Some(children) => children,
            },

            instance_prototypical_common_properties: Rc::new(RefCell::new(args.common_properties)),
            instance_prototypical_properties: Rc::new(RefCell::new(args.properties)),

            // source_expression_vec: args.repeat_source_expression_vec,
            // source_expression_range: args.repeat_source_expression_range,
            // active_children: Rc::new(RefCell::new(vec![])),
            // cleanup_children: Rc::new(RefCell::new(vec![])),
            // cached_old_value_vec: None,
            // cached_old_value_range: None,
        }));

        node_registry.register(instance_id, Rc::clone(&ret) as InstanceNodePtr<R>);
        ret
    }

    fn expand_node_and_compute_properties(&mut self, ptc: &mut PropertiesTreeContext<R>) -> Rc<RefCell<ExpandedNode<R>>> {

        let this_expanded_node = ExpandedNode::get_or_create_with_prototypical_properties(ptc, &self.instance_prototypical_properties, &self.instance_prototypical_common_properties);
        let properties_wrapped =  this_expanded_node.borrow().get_properties();

        //Mark all of Repeat's existing children (from previous tick) for unmount.  Then, when we iterate and append_children below, ensure that the mark-for-unmount is reverted
        //(See internals of `ExpandedNode#append_child_expanded_node` where we revert this unmounting.)  Revisit this logic in the presence of dirty-DAG.
        for cen in this_expanded_node.borrow().get_children_expanded_nodes() {
            ptc.engine.node_registry.borrow_mut().mark_for_unmount(cen.borrow().id_chain.clone());
        }

        with_properties_unsafe!(&properties_wrapped, PropertiesCoproduct, RepeatProperties, |properties: &mut RepeatProperties| {
            handle_vtable_update_optional!(ptc, properties.source_expression_range, std::ops::Range<isize>);
            handle_vtable_update_optional!(ptc, properties.source_expression_vec, std::vec::Vec<std::rc::Rc<core::cell::RefCell<PropertiesCoproduct>>>);

            if let Some(ref source) = properties.source_expression_range {
                let range_evaled = source.get();
                for i in range_evaled.start..range_evaled.end {
                    todo!("push stack frame, recurse, pop stack frame");
                    todo!("manage appending children");
                }

            } else if let Some(ref source) = properties.source_expression_vec {
                let vec_evaled = source.get();

                for pc in vec_evaled.iter() {
                    ptc.push_stack_frame(Rc::clone(pc));

                    for repeated_template_instance_root in self.repeated_template.borrow().iter() {
                        let mut new_ptc = ptc.clone();
                        new_ptc.current_expanded_node = None;
                        new_ptc.current_instance_node = Rc::clone(repeated_template_instance_root);

                        
                    }

                    ptc.pop_stack_frame()
                }
            }
        });


        // let (is_dirty, normalized_vec_of_props) = if let Some(se) = &self.source_expression_vec {
        //     //Handle case where the source expression is a Vec<Property<T>>,
        //     // like `for elem in self.data_list`
        //     let new_value = if let Some(tc) = ptc.compute_vtable_value(se._get_vtable_id().clone())
        //     {
        //         if let TypesCoproduct::stdCOCOvecCOCOVecLABRstdCOCOrcCOCORcLABRPropertiesCoproductRABRRABR(vec) = tc { vec } else { unreachable!() }
        //     } else {
        //         se.get().clone()
        //     };
        //
        //     let is_dirty = true;
        //     // self.cached_old_bounds = rtc.bounds.clone();
        //     // self.cached_old_value_vec = Some(new_value.clone());
        //     (is_dirty, new_value)
        // } else if let Some(se) = &self.source_expression_range {
        //     //Handle case where the source expression is a Range,
        //     // like `for i in 0..5`
        //     let new_value = if let Some(tc) = ptc.compute_vtable_value(se._get_vtable_id().clone())
        //     {
        //         if let TypesCoproduct::stdCOCOopsCOCORangeLABRisizeRABR(vec) = tc {
        //             vec
        //         } else {
        //             unreachable!()
        //         }
        //     } else {
        //         unreachable!()
        //     };
        //
        //     //Major hack: will only consider a new vec dirty if its cardinality changes.
        //     let is_dirty = true;
        //     // self.cached_old_bounds = rtc.bounds.clone();
        //     // self.cached_old_value_range = Some(new_value.clone());
        //     let normalized_vec_of_props = new_value
        //         .into_iter()
        //         .enumerate()
        //         .map(|(_i, elem)| Rc::new(PropertiesCoproduct::isize(elem)))
        //         .collect();
        //     (is_dirty, normalized_vec_of_props)
        // } else {
        //     unreachable!()
        // };

        // if is_dirty {
        //     //Any stated children (repeat template members) of Repeat should be forwarded to the `RepeatItem`-wrapped `ComponentInstance`s
        //
        //     if true {
        //         todo!("forward slot children; expose method of retrieving this");
        //         // let forwarded_slot_children = Rc::clone(&ptc.current_containing_component_slot_children);
        //     }
        //
        //     // let mut node_registry = (*rtc.engine.node_registry).borrow_mut();
        //     //
        //     // (*self.active_children)
        //     //     .borrow_mut()
        //     //     .iter()
        //     //     .for_each(|child| {
        //     //         let instance_id = (*(*child)).borrow_mut().get_instance_id();
        //     //         node_registry.deregister(instance_id);
        //     //         node_registry.mark_for_unmount(instance_id);
        //     //     });
        //     //
        //     // self.cleanup_children = self.active_children.clone();
        //
        //     //reset children:
        //     //wrap source_expression into `RepeatItems`, which attach
        //     //the necessary data as stack frame context
        //     self.active_children = Rc::new(RefCell::new(
        //         normalized_vec_of_props
        //             .iter()
        //             .enumerate()
        //             .map(|(i, datum)| {
        //                 // let instance_id = node_registry.mint_instance_id();
        //                 // let common_properties = CommonProperties::default();
        //
        //
        //                 let properties_for_stack_frame = Rc::new(RefCell::new(PropertiesCoproduct::RepeatItem(
        //                     Rc::clone(datum),
        //                     i,
        //                 )));
        //
        //                 todo!("loop over data; push stack frame; recurse compute_properties_recursive into (singular, shared) template tree")
        //
        //                 // let new_component_instance = ComponentInstance {
        //                 //     instance_id,
        //                 //     slot_children: Rc::clone(&forwarded_slot_children),
        //                 //     template: Rc::clone(&self.repeated_template),
        //                 //     common_properties,
        //                 //     properties: Rc::new(RefCell::new(PropertiesCoproduct::RepeatItem(
        //                 //         Rc::clone(datum),
        //                 //         i,
        //                 //     ))),
        //                 //     timeline: None,
        //                 //     handler_registry: None,
        //                 //     compute_properties_fn: Box::new(|_props, _rtc| {
        //                 //         //no-op since the Repeat RenderNode handles the necessary calc (see `RepeatInstance::compute_properties`)
        //                 //     }),
        //                 // };
        //
        //                 // let render_node: InstanceNodePtr<R> = Rc::new(RefCell::new(new_component_instance));
        //
        //                 // node_registry.register(instance_id, Rc::clone(&render_node));
        //                 // node_registry.mark_mounted(rtc.get_id_chain(instance_id));
        //
        //                 // (&*render_node).borrow_mut().mount_recursive(rtc);
        //
        //                 // render_node
        //             })
        //             .collect(),
        //     ));
        // }

        todo!()
    }

    fn is_invisible_to_slot(&self) -> bool {
        true
    }

    fn get_instance_children(&self) -> InstanceNodePtrList<R> {
        Rc::clone(&self.repeated_template)
    }

    fn get_layer_type(&mut self) -> Layer {
        Layer::DontCare
    }

    fn manages_own_subtree_for_expansion(&self) -> bool {
        true
    }
    // fn handle_mount(&mut self, ptc: &mut PropertiesTreeContext<R>) {
    //     // self.cached_old_value_range = None;
    //     // self.cached_old_value_vec = None;
    // }
}