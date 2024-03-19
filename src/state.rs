use gpui::*;
use std::{any::{ Any, TypeId }, collections::HashMap};

#[derive(Clone)]
pub struct ComponentState {
    states: Model<HashMap<&'static str, Box<dyn Any>>>,
    types: HashMap<&'static str, TypeId>,
}

impl ComponentState {
    pub fn init(cx: &mut WindowContext) -> Self {
        let this = Self {
            states: cx.new_model(|_| HashMap::new()),
            types: HashMap::new(),
        };

        cx.set_global(this.clone());

        this
    }

    pub fn add<T: 'static + Any + Clone + core::fmt::Debug>(
        key: &'static str,
        initial_value: T,
        cx: &mut WindowContext
    ) -> Option<T> {
        if !cx.has_global::<Self>() {
            return None;
        }

        let type_id = TypeId::of::<T>();

        let mut existing: Option<T> = None;

        cx.update_global::<Self, _>(|this, cx| {

            println!("states types: {:?}", this.types);

            if !this.types.contains_key(key) || this.types[key] != type_id {
                println!("!existed {}", key);
                this.states.update(cx, |this, cx| {
                    this.insert(key, Box::new(initial_value.clone()));
                });
                this.types.insert(key, type_id);
            } else if let Some(boxed_any) = this.states.read(cx).get(key) {
                println!("existed {}", key);
                if let Some(value) = boxed_any.downcast_ref::<T>() {
                    existing = Some(value.clone());
                }
            }
        });

        println!("current value: {:?}", existing.clone().or(Some(initial_value.clone())));

        existing.or(Some(initial_value))
    }

    pub fn update<T: 'static + Any>(
        key: &'static str,
        f: impl FnOnce(&mut T, &mut ModelContext<'_, HashMap<&str, Box<dyn Any>>>),
        cx: &mut WindowContext,
    ) {
        if !cx.has_global::<Self>() {
            return;
        }

        let type_id = TypeId::of::<T>();

        cx.update_global::<Self, _>(|this, _cx| {
            if let Some(&stored_type_id) = this.types.get(key) {
                if stored_type_id == type_id {
                    this.states.update(_cx, |this, _cx| {
                        if let Some(any) = this.get_mut(key) {
                            if let Some(value) = any.downcast_mut::<T>() {
                                f(value, _cx);
                            }
                        }
                    })
                }
            }
        });
    }
}

impl Global for ComponentState {}
