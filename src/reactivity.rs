slotmap::new_key_type! {
    pub struct ReactivityNodeKey;
}

pub struct ReactivityNode {
    generation: u64,
    dirty: bool,
    dependents: Vec<(u64, ReactivityNodeKey)>,
}

pub struct ReactivityNodes(slotmap::SlotMap<ReactivityNodeKey, ReactivityNode>);

impl ReactivityNodes {
    pub fn new() -> Self {
        ReactivityNodes(slotmap::SlotMap::with_key())
    }

    pub fn insert_new(&mut self) -> ReactivityNodeKey {
        self.0.insert(ReactivityNode {
            generation: 0,
            dirty: false,
            dependents: Vec::new(),
        })
    }

    pub fn mark_dirty(&mut self, key: ReactivityNodeKey) {
        // todo!()
    }
}

pub struct Ctx<'a> {
    reactivity_nodes: &'a mut ReactivityNodes,
    dependent: ReactivityNodeKey,
}

// This is loosely inspired by the paper Purely Functional Incremental Computing. The big difference
// is that we're doing reactivity tracking on top. The main thing we gain from this is more
// efficient list updates and it spares us having to assign unique identifiers to list entries.
pub trait Changeable: 'static + Copy {
    type Change: Clone + Default;
    type Reactivity: Copy;
    type Read: Copy;

    fn init_reactivity(&self, reactivity_nodes: &mut ReactivityNodes) -> Self::Reactivity;

    fn read(self, reactivity: Self::Reactivity) -> Self::Read;

    fn apply(
        self,
        reactivity: &mut Self::Reactivity,
        reactivity_nodes: &mut ReactivityNodes,
        change: Self::Change,
    ) -> Self;
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
    fn apply(
        self,
        value: T,
        reactivity: ReactivityNodeKey,
        reactivity_nodes: &mut ReactivityNodes,
    ) -> T {
        match self {
            PrimitiveChange::Keep => value,
            PrimitiveChange::ReplaceBy(new_value) => {
                reactivity_nodes.mark_dirty(reactivity);

                new_value
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct PrimitiveRead<T: Copy> {
    value: T,
    node: ReactivityNodeKey,
}

macro_rules! impl_primitive_change {
    ($t:ty) => {
        impl Changeable for $t {
            type Change = PrimitiveChange<$t>;
            type Reactivity = ReactivityNodeKey;
            type Read = PrimitiveRead<$t>;

            fn init_reactivity(&self, reactivity_nodes: &mut ReactivityNodes) -> Self::Reactivity {
                reactivity_nodes.insert_new()
            }

            fn read(self, reactivity: Self::Reactivity) -> Self::Read {
                PrimitiveRead {
                    value: self,
                    node: reactivity,
                }
            }

            fn apply(
                self,
                reactivity: &mut Self::Reactivity,
                reactivity_nodes: &mut ReactivityNodes,
                change: Self::Change,
            ) -> Self {
                change.apply(self, *reactivity, reactivity_nodes)
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

// Can I have derive macros?
//
// No, we have macros at home.
//
// Macros at home:
#[macro_export]
macro_rules! changeable_struct {
    (
        $vis:vis $name:ident,
        $changeable_name:ident,
        $reactivity_name:ident,
        $read_name:ident,
        { $($field_visibility:vis $field_name:ident: $field_type:ty,)* }
    ) => {
        #[derive(Copy, Clone)]
        $vis struct $name {
            $(
                $field_visibility $field_name: $field_type,
            )*
        }

        #[derive(Clone)]
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

        #[derive(Copy, Clone)]
        $vis struct $reactivity_name {
            $(
                $field_visibility $field_name:
                    <$field_type as $crate::reactivity::Changeable>::Reactivity,
            )*
        }

        #[derive(Copy, Clone)]
        $vis struct $read_name {
            $(
                $field_visibility $field_name:
                    <$field_type as $crate::reactivity::Changeable>::Read,
            )*
        }

        impl $crate::reactivity::Changeable for $name {
            type Change = $changeable_name;
            type Reactivity = $reactivity_name;
            type Read = $read_name;

            fn init_reactivity(&self, reactivity_nodes: &mut ReactivityNodes) -> Self::Reactivity {
                $reactivity_name {
                    $($field_name: self.$field_name.init_reactivity(reactivity_nodes),)*
                }
            }

            fn read(self, reactivity: Self::Reactivity) -> Self::Read {
                $read_name {
                    $($field_name: self.$field_name.read(reactivity.$field_name),)*
                }
            }

            fn apply(
                self,
                reactivity: &mut Self::Reactivity,
                reactivity_nodes: &mut $crate::reactivity::ReactivityNodes,
                change: Self::Change,
            ) -> Self {
                Self {
                    $($field_name: self.$field_name.apply(
                        &mut reactivity.$field_name,
                        reactivity_nodes,
                        change.$field_name,
                    ),)*
                }
            }
        }
    };
}
