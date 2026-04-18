// This is loosely inspired by the paper Purely Functional Incremental Computing. The big difference
// is that we're doing reactivity tracking on top. The main thing we gain from this is more
// efficient list updates and it spares us having to assign unique identifiers to list entries.
pub trait Changeable: 'static + Copy {
    type Change: Clone + Default;

    fn apply(self, change: Self::Change) -> Self;
}

#[derive(Clone)]
pub enum PrimitiveChange<T> {
    Keep,
    ReplaceBy(T),
}

impl<T> Default for PrimitiveChange<T> {
    fn default() -> Self {
        PrimitiveChange::Keep
    }
}

impl<T> PrimitiveChange<T> {
    fn apply(self, value: T) -> T {
        match self {
            PrimitiveChange::Keep => value,
            PrimitiveChange::ReplaceBy(new_value) => new_value,
        }
    }
}

macro_rules! impl_primitive_change {
    ($t:ty) => {
        impl Changeable for $t {
            type Change = PrimitiveChange<$t>;

            fn apply(self, change: Self::Change) -> Self {
                change.apply(self)
            }
        }
    };
}

impl_primitive_change!(bool);
impl_primitive_change!(u8);
impl_primitive_change!(i8);
impl_primitive_change!(u16);
impl_primitive_change!(i16);
impl_primitive_change!(u32);
impl_primitive_change!(i32);
impl_primitive_change!(u64);
impl_primitive_change!(i64);
impl_primitive_change!(u128);
impl_primitive_change!(i128);
impl_primitive_change!(&'static str);

/// Can we have derive macros?
///
/// No, we have macros at home.
///
/// Macros at home:
#[macro_export]
macro_rules! changeable_struct {
    (
        $vis:vis $name:ident,
        $changeable_name:ident,
        { $($field_visibility:vis $field_name:ident: $field_type:ty,)* }
    ) => {
        $vis struct $name {
            $(
                $field_visibility $field_name: $field_type,
            )*
        }

        $vis struct $changeable_name {
            $(
                $field_visibility $field_name:
                    <$field_type as $crate::reactivity::Changeable>::Change,
            )*
        }

        impl std::default::Default for $changeable_name {
            fn default() -> Self {
                Self {
                    $($field_name: std::default::Default::default(),)*
                }
            }
        }

        impl $crate::reactivity::Changeable for $name {
            type Change = $changeable_name;

            fn apply(&mut self, change: Self::Change) {
                $(
                    $crate::reactivity::Changeable::apply(
                        &mut self.$field_name,
                        change.$field_name,
                    );
                )*
            }
        }
    };
}
