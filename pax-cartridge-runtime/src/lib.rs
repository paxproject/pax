use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use pax_core::{ComponentInstance, PropertyExpression, RenderNodePtrList, RenderTreeContext, ExpressionContext, PaxEngine, RenderNode, InstanceMap, HandlerRegistry, InstantiationArgs, ConditionalInstance, PlaceholderInstance};
use pax_core::pax_properties_coproduct::{PropertiesCoproduct, TypesCoproduct};
use pax_core::repeat::{RepeatInstance};

use pax_runtime_api::{ArgsCoproduct, PropertyInstance, PropertyLiteral, Size2D, Transform2D};

//generate dependencies, pointing to userland cartridge (same logic as in PropertiesCoproduct)
use pax_example::pax_types::{Root};
use pax_example::pax_types::pax_std::primitives::{Rectangle, Group};
use pax_example::pax_types::pax_std::types::{Color, Stroke, Size, SpreadCellProperties};
use pax_example::pax_types::pax_std::components::Spread;

//dependency paths below come from pax_primitive macro, where these crate+module paths are passed as parameters:
use pax_std_primitives::{RectangleInstance, GroupInstance, FrameInstance};


pub fn instantiate_expression_table() -> HashMap<String, Box<dyn Fn(ExpressionContext) -> TypesCoproduct>> {
    let mut map : HashMap<String, Box<dyn Fn(ExpressionContext) -> TypesCoproduct>> = HashMap::new();

    //literal string IDs to be generated by compiler

    //note that type coercion should happen here, too:
    //(must know symbol name as well as source & destination types)
    //(compiler can keep a dict of operand types)

    map.insert("a".to_string(), Box::new(|ec: ExpressionContext| -> TypesCoproduct {
        let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let x = (*ec.engine).borrow();
            (Rc::clone(datum), *i)
        } else { unreachable!() };

        #[allow(non_snake_case)]
        let __AT__frames_elapsed = ec.engine.frames_elapsed as f64;
        let i = i as f64;

        // datum

        //#[allow(non_snake_case)]
        // let self__DOT__rotation = if let PropertiesCoproduct::Root(p) = &*(*(*(*ec.stack_frame).borrow().get_scope()).borrow().properties).borrow() {
        //     *p.current_rotation.get()
        // } else { unreachable!() };

        return TypesCoproduct::Transform2D(
            Transform2D::align(Size::Percent(50.0), Size::Percent(50.0)) *
            Transform2D::origin(Size::Percent(50.0), Size::Percent(50.0)) *
            Transform2D::rotate(__AT__frames_elapsed * i / 100.0) *
            Transform2D::translate(i * 10.0, i * 10.0) *
            Transform2D::rotate(__AT__frames_elapsed / 50.0)

        )
        // } else {unreachable!()};

    }));


    map.insert("c".to_string(), Box::new(|ec: ExpressionContext| -> TypesCoproduct {
        #[allow(non_snake_case)]
        let __AT__frames_elapsed = ec.engine.frames_elapsed as f64;

        //TODO: how to determine (for Expression codegen) that StrokeProperties is compound and requires
        //      wrapping in PropertyLiteral values?
        TypesCoproduct::Stroke(
            Stroke {
                color: Box::new(PropertyLiteral (Color::hlca((__AT__frames_elapsed as isize % 360) as f64, 100.0,100.0,1.0) )),
                width: Box::new(PropertyLiteral (45.0)),
            }
        )
    }));

    //this expression handles re-packing `data_list` for
    //`@for (elem, i) in computed_layout_spec {`
    map.insert("f".to_string(), Box::new(|ec: ExpressionContext| -> TypesCoproduct {

        //get computed_layout_spec, which is a Vec<Rc<SpreadCellProperties>>
        //map (enumerate) its elements as elem into (elem, i),

        // let computed_layout_spec = : Vec<SpreadCellProperties>


        //#[allow(non_snake_case)]
        // let computed_layout_spec = if let PropertiesCoproduct::Root(p) = &*(*(*(*ec.stack_frame).borrow().get_scope()).borrow().properties).borrow() {
        //     *p.current_rotation.get()
        // } else { unreachable!() };


        //note this unwrapping is nested inside the `if let`, rather than flatted into a single assignment.
        //This is necessary for the non-clonable `Vec` in this case, and might need/want to be applied across codegen
        //(nesting instead of implicit cloning, e.g. of primitive types)
        #[allow(non_snake_case)]
        if let PropertiesCoproduct::Spread(p) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let computed_layout_spec = p.computed_layout_spec.get();
            return TypesCoproduct::Vec_Rc_PropertiesCoproduct___(computed_layout_spec.iter().enumerate().map(|(i,e)|{
                let cloned = Rc::clone(e);
                //TODO: there should be a way to pull off this re-wrapping without cloning the data structure (below).  One option is to deal with raw refs to the datum (we
                //are guaranteed immutable reads for this data, after all.)
                let rewrapped = PropertiesCoproduct::SpreadCellProperties((*cloned).clone());
                Rc::new(rewrapped)
            }).collect());
        } else { unreachable!() };

        //we might need to wrap SpreadCellProperties in `Rc`s so that we can we can easily clone
        //them into RepeatItems. Alternatively, perhaps RepeatItem can accept raw references...?

        //now wrap each element into a PropertiesCoproduct::RepeatItem, so that we have access to (elem, i)
        //in child scopes

    }));

    map.insert("g".to_string(), Box::new(|ec: ExpressionContext| -> TypesCoproduct {
        let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let x = (*ec.engine).borrow();
            (Rc::clone(datum), *i)
        } else { unreachable!("alpha") };

        let datum_cast = if let PropertiesCoproduct::SpreadCellProperties(d)= &*datum {d} else {unreachable!("beta")};


        return TypesCoproduct::Transform2D(
            Transform2D::translate(datum_cast.x_px, datum_cast.y_px)
        )

    }));

    //Frame size x
    map.insert("h".to_string(), Box::new(|ec: ExpressionContext| -> TypesCoproduct {
        let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let x = (*ec.engine).borrow();
            (Rc::clone(datum), *i)
        } else { unreachable!("gamma") };

        let datum_cast = if let PropertiesCoproduct::SpreadCellProperties(d)= &*datum {d} else {unreachable!("epsilon")};
        // (*ec.engine.runtime).borrow().log(&format!("evaling layout width {}", datum_cast.width_px));
        return TypesCoproduct::Size(
            Size::Pixel(datum_cast.width_px)
        )
    }));

    //Frame size y
    map.insert("i".to_string(), Box::new(|ec: ExpressionContext| -> TypesCoproduct {
        let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let x = (*ec.engine).borrow();
            (Rc::clone(datum), *i)
        } else { unreachable!("delta") };

        let datum_cast = if let PropertiesCoproduct::SpreadCellProperties(d)= &*datum {d} else {unreachable!()};

        return TypesCoproduct::Size(
            Size::Pixel(datum_cast.height_px)
        )
    }));

    //Frame index
    map.insert("j".to_string(), Box::new(|ec: ExpressionContext| -> TypesCoproduct {
        let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let x = (*ec.engine).borrow();
            (Rc::clone(datum), *i)
        } else { unreachable!("epsilon") };

        return TypesCoproduct::usize(
            i
        );
    }));

    map
}

pub fn instantiate_root_component(instance_map: Rc<RefCell<InstanceMap>>) -> Rc<RefCell<ComponentInstance>> {
    //Root
    ComponentInstance::instantiate(
        InstantiationArgs{
            properties: PropertiesCoproduct::Root(Root {
                //these values are code-genned by pax-compiler.  If not provided, pax-compiler
                //can inject Default::default.  If the rust compiler throws an error,
                //that is the user's responsibility.
                num_clicks: Default::default(),
                current_rotation: Box::new(PropertyLiteral(0.0)),
            }),
            handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                click_handlers: vec![],
                pre_render_handlers: vec![
                    |properties,args|{
                        let properties = &mut *properties.as_ref().borrow_mut();
                        let properties = if let PropertiesCoproduct::Root(p) = properties {p} else {unreachable!()};
                        Root::handle_pre_render(properties, args);
                    }
                ]
            }))),
            instance_map: Rc::clone(&instance_map),
            transform: Transform2D::default_wrapped(),
            size: None,
            children: None,
            component_template: Some(Rc::new(RefCell::new(vec![

                //rainbow box
                // GroupInstance::instantiate(InstantiationArgs {
                //     properties: PropertiesCoproduct::None,
                //     handler_registry: None,
                //     instance_map: Rc::clone(&instance_map),
                //     transform: Transform2D::default_wrapped(),
                //     size: None,
                //     component_adoptees: None,
                //     component_template: None,
                //     repeat_data_list: None,
                //     conditional_boolean_expression: None,
                //     compute_properties_fn: None,
                //     primitive_children: Some(Rc::new(RefCell::new(vec![
                //         ConditionalInstance::instantiate(InstantiationArgs {
                //             properties: PropertiesCoproduct::None,
                //             handler_registry: None,
                //             instance_map: Rc::clone(&instance_map),
                //             transform: Transform2D::default_wrapped(),
                //             size: None,
                //             primitive_children: Some(Rc::new(RefCell::new(vec![
                //                 RectangleInstance::instantiate(InstantiationArgs {
                //                     properties: PropertiesCoproduct::Rectangle(Rectangle{
                //                         stroke: Box::new(PropertyExpression {id: "c".into(), cached_value: Default::default()}),
                //                         fill: Box::new(PropertyLiteral (Color::rgba(1.0, 1.0, 0.0, 1.0)))
                //                     }),
                //                     handler_registry: None,
                //                     instance_map: Rc::clone(&instance_map),
                //                     transform: Rc::new(RefCell::new(PropertyLiteral(Transform2D::translate(200.0, 200.0)))),
                //                     size: Some([PropertyLiteral(Size::Pixel(200.0)).into(),PropertyLiteral(Size::Pixel(200.0)).into()]),
                //                     primitive_children: None,
                //                     component_template: None,
                //                     component_adoptees: None,
                //                     placeholder_index: None,
                //                     repeat_data_list: None,
                //                     conditional_boolean_expression: None,
                //                     compute_properties_fn: None
                //                 }),
                //             ]))),
                //             component_template: None,
                //             component_adoptees: None,
                //             placeholder_index: None,
                //             repeat_data_list: None,
                //             conditional_boolean_expression: Some(Box::new(PropertyLiteral(true))),
                //             compute_properties_fn: None
                //         }),
                //     ]))),
                //
                //     placeholder_index: None
                // }),
                //end rainbow box
                //Spread
                ComponentInstance::instantiate(
                    InstantiationArgs {
                        properties: PropertiesCoproduct::Spread(Spread {
                            computed_layout_spec: Default::default(),
                            direction: Default::default(),
                            cell_count: Box::new(PropertyLiteral(10)),
                            gutter_width: Box::new(PropertyLiteral(Size::Pixel(5.0))),
                            overrides_cell_size: Default::default(),
                            overrides_gutter_size: Default::default(),
                        }),
                        handler_registry: Some(Rc::new(RefCell::new(
                            HandlerRegistry {
                                click_handlers: vec![],
                                pre_render_handlers: vec![
                                    |properties,args|{
                                        let properties = &mut *properties.as_ref().borrow_mut();
                                        let properties = if let PropertiesCoproduct::Spread(p) = properties {p} else {unreachable!()};
                                        Spread::handle_pre_render(properties, args);
                                    }
                                ],
                            }
                        ))),
                        instance_map: Rc::clone(&instance_map),
                        transform: Transform2D::default_wrapped(),
                        size: Some([Box::new(PropertyLiteral(Size::Percent(100.0))), Box::new(PropertyLiteral(Size::Percent(100.0)))]),
                        children: Some(Rc::new(RefCell::new(vec![
                            RectangleInstance::instantiate(InstantiationArgs{
                                properties: PropertiesCoproduct::Rectangle(Rectangle {
                                    stroke: Default::default(),
                                    fill: Box::new(PropertyLiteral(Color::rgba(0.0, 1.0, 1.0, 1.0)))
                                }),
                                handler_registry: None,
                                instance_map: Rc::clone(&instance_map),
                                transform: Transform2D::default_wrapped(),
                                size: Some([PropertyLiteral(Size::Percent(100.0)).into(),PropertyLiteral(Size::Percent(100.0)).into()]),
                                children: None,
                                component_template: None,
                                placeholder_index: None,
                                should_skip_adoption: false,
                                repeat_data_list: None,
                                conditional_boolean_expression: None,
                                compute_properties_fn: None
                            }),
                            RepeatInstance::instantiate(InstantiationArgs {
                                properties: PropertiesCoproduct::None,
                                handler_registry: None,
                                instance_map: Rc::clone(&instance_map),
                                transform: Transform2D::default_wrapped(),
                                size: None,
                                children: Some(Rc::new(RefCell::new( vec![
                                    RectangleInstance::instantiate(InstantiationArgs{
                                        properties: PropertiesCoproduct::Rectangle(Rectangle {
                                            stroke: Default::default(),
                                            fill: Box::new(PropertyLiteral(Color::rgba(1.0, 0.45, 0.25, 1.0)))
                                        }),
                                        handler_registry: None,
                                        instance_map: Rc::clone(&instance_map),
                                        transform: Transform2D::default_wrapped(),
                                        size: Some([PropertyLiteral(Size::Percent(100.0)).into(),PropertyLiteral(Size::Percent(100.0)).into()]),
                                        children: None,
                                        component_template: None,
                                        placeholder_index: None,
                                        should_skip_adoption: false,
                                        repeat_data_list: None,
                                        conditional_boolean_expression: None,
                                        compute_properties_fn: None
                                        })
                                    ]
                                ))),
                                component_template: None,
                                should_skip_adoption: false,
                                placeholder_index: None,
                                repeat_data_list: Some(Box::new(PropertyLiteral((0..8).into_iter().map(|i|{
                                    Rc::new(PropertiesCoproduct::isize(i))
                                }).collect()))),
                                conditional_boolean_expression: None,
                                compute_properties_fn: None
                            }),
                            RectangleInstance::instantiate(InstantiationArgs{
                                properties: PropertiesCoproduct::Rectangle(Rectangle {
                                    stroke: Default::default(),
                                    fill: Box::new(PropertyLiteral(Color::rgba(1.0, 1.0, 0.0, 1.0)))
                                }),
                                handler_registry: None,
                                instance_map: Rc::clone(&instance_map),
                                transform: Transform2D::default_wrapped(),
                                size: Some([PropertyLiteral(Size::Percent(100.0)).into(),PropertyLiteral(Size::Percent(100.0)).into()]),
                                children: None,
                                component_template: None,
                                placeholder_index: None,
                                should_skip_adoption: false,
                                repeat_data_list: None,
                                conditional_boolean_expression: None,
                                compute_properties_fn: None
                            }),
                        ]))),
                        component_template: Some(Rc::new(RefCell::new(
                            vec![
                                RepeatInstance::instantiate(InstantiationArgs {
                                    properties: PropertiesCoproduct::None,
                                    handler_registry: None,
                                    instance_map: Rc::clone(&instance_map),
                                    transform: Transform2D::default_wrapped(),
                                    size: None,
                                    component_template: None,
                                    children: Some(Rc::new(RefCell::new(vec![
                                        FrameInstance::instantiate(InstantiationArgs{
                                            properties: PropertiesCoproduct::None,
                                            handler_registry: None,
                                            instance_map: Rc::clone(&instance_map),
                                            transform: Rc::new(RefCell::new(PropertyExpression {
                                                id: "g".to_string(),
                                                cached_value: Transform2D::default(),
                                            })),
                                            size: Some([
                                                Box::new(PropertyExpression {
                                                    id: "h".to_string(),
                                                    cached_value: Default::default()
                                                }),
                                                Box::new(PropertyExpression {
                                                    id: "i".to_string(),
                                                    cached_value: Default::default()
                                                }
                                            )]),
                                            children: Some(Rc::new(RefCell::new(vec![
                                                PlaceholderInstance::instantiate(InstantiationArgs {
                                                    properties: PropertiesCoproduct::None,
                                                    handler_registry: None,
                                                    instance_map: Rc::clone(&instance_map),
                                                    transform: Transform2D::default_wrapped(),
                                                    size: Some([PropertyLiteral(Size::Percent(100.0)).into(),PropertyLiteral(Size::Percent(100.0)).into()]),
                                                    children: None,
                                                    component_template: None,
                                                    should_skip_adoption: false,
                                                    placeholder_index: Some(Box::new(PropertyExpression {
                                                        id: "j".to_string(),
                                                        cached_value: Default::default()
                                                    })),
                                                    repeat_data_list: None,
                                                    conditional_boolean_expression: None,
                                                    compute_properties_fn: None
                                                }),
                                                // RectangleInstance::instantiate(InstantiationArgs {
                                                //     properties: PropertiesCoproduct::Rectangle(Rectangle{
                                                //         stroke: Box::new(PropertyLiteral (Stroke {
                                                //             color: Box::new(PropertyLiteral(Color::rgba(1.0, 0.0, 1.0, 1.0))),
                                                //             width: Box::new(PropertyLiteral(5.0))
                                                //         })),
                                                //         fill: Box::new(PropertyLiteral (Color::rgba(1.0, 1.0, 0.0, 1.0)))
                                                //     }),
                                                //     handler_registry: None,
                                                //     instance_map: Rc::clone(&instance_map),
                                                //     transform: Transform2D::default_wrapped(),
                                                //     size: Some([PropertyLiteral(Size::Percent(100.0)).into(),PropertyLiteral(Size::Percent(100.0)).into()]),
                                                //     primitive_children: None,
                                                //     component_template: None,
                                                //     component_adoptees: None,
                                                //     placeholder_index: None,
                                                //     repeat_data_list: None,
                                                //     conditional_boolean_expression: None,
                                                //     compute_properties_fn: None
                                                // }),
                                            ]))),
                                            component_template: None,
                                            should_skip_adoption: false,
                                            placeholder_index: None,
                                            repeat_data_list: None,
                                            conditional_boolean_expression: None,
                                            compute_properties_fn: None
                                        }),
                                    ]))),
                                    placeholder_index: None,
                                    repeat_data_list: Some(Box::new(PropertyExpression {
                                        id: "f".to_string(),
                                        cached_value: vec![],
                                    })),
                                    conditional_boolean_expression: None,
                                    compute_properties_fn: None,
                                    should_skip_adoption: false
                                }),
                            ]
                        ))),

                        should_skip_adoption: false,
                        placeholder_index: None,
                        repeat_data_list: None,
                        conditional_boolean_expression: None,
                        compute_properties_fn: Some(Box::new(|properties, rtc|{
                            let properties = &mut *properties.as_ref().borrow_mut();
                            let properties = if let PropertiesCoproduct::Spread(p) = properties {p} else {unreachable!()};

                            if let Some(new_value) = rtc.get_computed_value(properties.direction._get_vtable_id()) {
                                let new_value = if let TypesCoproduct::SpreadDirection(v) = new_value { v } else { unreachable!() };
                                properties.direction.set(new_value);
                            }

                            if let Some(new_value) = rtc.get_computed_value(properties.cell_count._get_vtable_id()) {
                                let new_value = if let TypesCoproduct::usize(v) = new_value { v } else { unreachable!() };
                                properties.cell_count.set(new_value);
                            }

                            if let Some(new_value) = rtc.get_computed_value(properties.gutter_width._get_vtable_id()) {
                                let new_value = if let TypesCoproduct::Size(v) = new_value { v } else { unreachable!() };
                                properties.gutter_width.set(new_value);
                            }

                            if let Some(new_value) = rtc.get_computed_value(properties.overrides_cell_size._get_vtable_id()) {
                                let new_value = if let TypesCoproduct::Vec_LPAREN_usize_COMMA_Size_RPAREN(v) = new_value { v } else { unreachable!() };
                                properties.overrides_cell_size.set(new_value);
                            }

                            if let Some(new_value) = rtc.get_computed_value(properties.overrides_gutter_size._get_vtable_id()) {
                                let new_value = if let TypesCoproduct::Vec_LPAREN_usize_COMMA_Size_RPAREN(v) = new_value { v } else { unreachable!() };
                                properties.overrides_gutter_size.set(new_value);
                            }

                        }))
                    }
                ),
                //End Spread

            ]))),
            should_skip_adoption: false,
            placeholder_index: None,
            repeat_data_list: None,
            conditional_boolean_expression: None,
            compute_properties_fn: Some(Box::new(|properties, rtc|{
                let properties = &mut *properties.as_ref().borrow_mut();
                let properties = if let PropertiesCoproduct::Root(p) = properties {p} else {unreachable!()};

                if let Some(new_current_rotation) = rtc.get_computed_value(properties.current_rotation._get_vtable_id()) {
                    let new_value = if let TypesCoproduct::f64(v) = new_current_rotation { v } else { unreachable!() };
                    properties.current_rotation.set(new_value);
                }

                if let Some(new_num_clicks) = rtc.get_computed_value(properties.num_clicks._get_vtable_id()) {
                    let new_value = if let TypesCoproduct::isize(v) = new_num_clicks { v } else { unreachable!() };
                    properties.num_clicks.set(new_value);
                }

                // if let Some(new_deeper_struct) = rtc.get_computed_value(properties.deeper_struct._get_vtable_id()) {
                //     let new_value = if let TypesCoproduct::DeeperStruct(v) = new_deeper_struct { v } else { unreachable!() };
                //     properties.deeper_struct.set(new_value);
                // }
            }))
        }
    )
}