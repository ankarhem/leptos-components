use leptos::{create_rw_signal, RwSignal, Scope};
use uuid::Uuid;

#[derive(Clone)]
pub struct ListboxContext {
    pub(super) id: Uuid,
    pub(super) open: RwSignal<bool>,
    pub(super) active: RwSignal<Option<Uuid>>,
    pub(super) button_id: Uuid,
}

#[derive(Clone, Copy)]
pub struct ListboxValue<T>(pub(super) RwSignal<T>)
where
    T: 'static + Clone + Copy + PartialEq;

impl ListboxContext {
    pub(super) fn new(cx: Scope) -> Self {
        Self {
            id: Uuid::new_v4(),
            open: create_rw_signal(cx, false),
            active: create_rw_signal(cx, None),
            button_id: Uuid::new_v4(),
        }
    }
}
