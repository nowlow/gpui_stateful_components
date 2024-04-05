use gpui::*;
// use uuid::Uuid;
use hex;
use std::{any::{ Any, TypeId }, collections::HashMap};

#[derive(Clone, Copy)]
pub struct State<T> {
    value: Option<T>,
    pub key: [u8; 16],
}

impl<T: 'static + Clone + Any> State<T> {
    pub fn new(value: T, key: [u8; 16], cx: &mut WindowContext) -> Self {
        let mut state = State {
            value: None,
            // key: Uuid::new_v4().into_bytes(),
            key, // FIXME: use uuid instead, but it's unreliable since this will be rebuilt every time the component is rerendered
        };

        state.value = StateModel::add(state.key, value, cx);

        state
    }

    pub fn update(self, value: T, cx: &mut WindowContext) {
        StateModel::update(self.key, |this, _| *this = value, cx)
    }

    pub fn get(self) -> Option<T> {
        self.value
    }

    pub fn delete(self, cx: &mut WindowContext) {
        StateModel::delete(self.key, cx)
    }
}

#[derive(Clone)]
pub struct StateModel {
    states: Model<HashMap<String, Box<dyn Any>>>,
    types: HashMap<String, TypeId>,
}

impl StateModel {
    pub fn init(cx: &mut WindowContext) -> Self {
        let this = Self {
            states: cx.new_model(|_| HashMap::new()),
            types: HashMap::new(),
        };

        cx.set_global(this.clone());

        this
    }

    pub fn add<T: 'static + Any + Clone>(
        raw_key: [u8; 16],
        initial_value: T,
        cx: &mut WindowContext
    ) -> Option<T> {
        if !cx.has_global::<Self>() {
            return None;
        }

        let key = hex::encode(raw_key);

        let type_id = TypeId::of::<T>();

        let mut existing: Option<T> = None;

        cx.update_global::<Self, _>(|this, cx| {

            // println!("states types: {:?}", this.types);

            if !this.types.contains_key(&key) || this.types[&key] != type_id {
                this.states.update(cx, |this, _cx| {
                    this.insert(key.clone(), Box::new(initial_value.clone()));
                });
                this.types.insert(key, type_id);
            } else if let Some(boxed_any) = this.states.read(cx).get(&key) {
                if let Some(value) = boxed_any.downcast_ref::<T>() {
                    existing = Some(value.clone());
                }
            }
        });

        existing.or(Some(initial_value))
    }

    pub fn update<T: 'static + Any>(
        raw_key: [u8; 16],
        f: impl FnOnce(&mut T, &mut ModelContext<'_, HashMap<String, Box<dyn Any>>>),
        cx: &mut WindowContext,
    ) {
        if !cx.has_global::<Self>() {
            return;
        }

        let key = hex::encode(raw_key);

        let type_id = TypeId::of::<T>();

        cx.update_global::<Self, _>(|this, _cx| {
            if let Some(&stored_type_id) = this.types.get(&key) {
                if stored_type_id == type_id {
                    this.states.update(_cx, |this, _cx| {
                        if let Some(any) = this.get_mut(&key) {
                            if let Some(value) = any.downcast_mut::<T>() {
                                f(value, _cx);
                            }
                        }
                    })
                }
            }
        });
    }

    pub fn delete(
        raw_key: [u8; 16],
        cx: &mut WindowContext,
    ) {
        if !cx.has_global::<Self>() {
            return;
        }

        let key = hex::encode(raw_key);

        cx.update_global::<Self, _>(|this, _cx| {
            this.states.update(_cx, |this, _cx| {
                this.remove(&key);
            });
            this.types.remove(&key);
        });
    }

    pub fn exists(raw_key: [u8; 16], cx: &mut WindowContext) -> bool {
        if !cx.has_global::<Self>() {
            return false;
        }

        let key = hex::encode(raw_key);

        let global = cx.global::<Self>();

        global.types.contains_key(&key)
    }
}

impl Global for StateModel {}

