use gpui::*;
// use uuid::Uuid;
use hex;
use std::{any::{ Any, TypeId }, collections::HashMap};

#[derive(Clone, Copy)]
pub struct State<T> {
    value: Option<T>,
    key: [u8; 16],
}

impl<T: 'static + Clone + Any> State<T> {
    pub fn new(value: T, key: [u8; 16], cx: &mut WindowContext) -> Self {
        let mut state = State {
            value: None,
            // key: Uuid::new_v4().into_bytes(),
            key, // FIXME: use uuid instead, but it's unreliable since this will be rebuilt every time the component is rerendered
        };

        state.value = ComponentState::add(state.key, value, cx);

        state
    }

    pub fn update(self, value: T, cx: &mut WindowContext) {
        ComponentState::update(self.key, |this, _| *this = value, cx)
    }

    pub fn get(self) -> Option<T> {
        self.value
    }
}

#[derive(Clone)]
pub struct ComponentState {
    states: Model<HashMap<String, Box<dyn Any>>>,
    types: HashMap<String, TypeId>,
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

            println!("states types: {:?}", this.types);

            if !this.types.contains_key(&key) || this.types[&key] != type_id {
                println!("!existed {}", key);
                this.states.update(cx, |this, _| {
                    this.insert(key.clone(), Box::new(initial_value.clone()));
                });
                this.types.insert(key, type_id);
            } else if let Some(boxed_any) = this.states.read(cx).get(&key) {
                println!("existed {}", key);
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
}

impl Global for ComponentState {}
