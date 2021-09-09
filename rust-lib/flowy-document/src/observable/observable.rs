use flowy_derive::ProtoBuf_Enum;
use flowy_observable::ObservableBuilder;
const OBSERVABLE_CATEGORY: &'static str = "Doc";
#[derive(ProtoBuf_Enum, Debug)]
pub(crate) enum DocObservable {
    UserCreateDoc = 0,
}

impl std::convert::Into<i32> for DocObservable {
    fn into(self) -> i32 { self as i32 }
}

pub(crate) fn observable(id: &str, ty: DocObservable) -> ObservableBuilder { ObservableBuilder::new(id, ty, OBSERVABLE_CATEGORY) }
