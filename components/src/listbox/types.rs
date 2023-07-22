use leptos::{create_rw_signal, RwSignal, Scope};
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct ListboxContext {
    pub(super) id: Uuid,
    pub(super) open: RwSignal<bool>,
    pub(super) selected: RwSignal<Option<Uuid>>,
    pub(super) active: RwSignal<Option<Uuid>>,
}

impl ListboxContext {
    pub(crate) fn new(cx: Scope) -> Self {
        Self {
            id: Uuid::new_v4(),
            open: create_rw_signal(cx, false),
            selected: create_rw_signal(cx, None),
            active: create_rw_signal(cx, None),
        }
    }
}
