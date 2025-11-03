use crate::{Borrow, BorrowMut, Error, PtrMut, Result, prelude::*};
use indexmap::{Equivalent, IndexMap};
use koto_memory::OptPtrMut;
use rustc_hash::FxHasher;
use std::{
    hash::{BuildHasherDefault, Hash, Hasher},
    ops::{Deref, DerefMut, RangeBounds},
};

/// The hasher used throughout the Koto runtime
#[derive(Default)]
pub struct KotoHasher(FxHasher);

#[cfg(any(feature = "gc", feature = "agc"))]
unsafe impl<V: koto_memory::Visitor> koto_memory::TraceWith<V> for KotoHasher {
    fn accept(&self, _visitor: &mut V) -> core::result::Result<(), ()> {
        Ok(())
    }
}

impl Hasher for KotoHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.0.write_u8(i)
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.0.write_u16(i)
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.0.write_u32(i)
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0.write_u64(i)
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.0.write_u128(i)
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.0.write_usize(i)
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
}

type ValueMapType = IndexMap<ValueKey, KValue, BuildHasherDefault<KotoHasher>>;

/// The (ValueKey -> Value) 'data' hash map used by the Koto runtime
///
/// See also: [KMap]
#[derive(Clone, Default)]
pub struct ValueMap(ValueMapType);

impl ValueMap {
    /// Creates a new map with the given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self(ValueMapType::with_capacity_and_hasher(
            capacity,
            Default::default(),
        ))
    }

    /// Creates a new map containing a slice of the map's elements
    pub fn make_data_slice(&self, range: impl RangeBounds<usize>) -> Option<Self> {
        self.get_range(range).map(|entries| {
            Self::from_iter(
                entries
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone())),
            )
        })
    }
}

impl Deref for ValueMap {
    type Target = ValueMapType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<(ValueKey, KValue)> for ValueMap {
    fn from_iter<T: IntoIterator<Item = (ValueKey, KValue)>>(iter: T) -> ValueMap {
        Self(ValueMapType::from_iter(iter))
    }
}

#[cfg(any(feature = "gc", feature = "agc"))]
unsafe impl<V: koto_memory::Visitor> koto_memory::TraceWith<V> for ValueMap {
    fn accept(&self, visitor: &mut V) -> core::result::Result<(), ()> {
        for (key, value) in &self.0 {
            key.accept(visitor)?;
            value.accept(visitor)?;
        }

        self.hasher().accept(visitor)
    }
}

/// The core hash map value type used in Koto, containing a [ValueMap] and a [MetaMap]
#[derive(Clone, Default, KotoTrace)]
#[koto(runtime = crate)]
pub struct KMap {
    data: PtrMut<ValueMap>,
    meta: OptPtrMut<MetaMap>,
}

impl KMap {
    /// Creates an empty KMap
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty KMap, with a MetaMap containing the given @type string
    pub fn with_type(type_name: &str) -> Self {
        let mut meta = MetaMap::default();
        meta.insert(MetaKey::Type, type_name.into());
        Self::with_contents(ValueMap::default(), Some(meta))
    }

    /// Creates an empty KMap with the given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_contents(ValueMap::with_capacity(capacity), None)
    }

    /// Creates a KMap initialized with the provided data
    pub fn with_data(data: ValueMap) -> Self {
        Self::with_contents(data, None)
    }

    /// Creates a KMap initialized with the provided data and meta map
    pub fn with_contents(data: ValueMap, meta: Option<MetaMap>) -> Self {
        Self {
            data: data.into(),
            meta: meta.map(PtrMut::from).into(),
        }
    }

    /// Makes a KMap taking the data map from the first arg, and the meta map from the second
    pub fn from_data_and_meta_maps(data: &Self, meta: &Self) -> Self {
        Self {
            data: data.data.clone(),
            meta: meta.meta.clone(),
        }
    }

    /// Provides a reference to the data map
    pub fn data(&self) -> Borrow<'_, ValueMap> {
        self.data.borrow()
    }

    /// Provides a mutable reference to the data map
    pub fn data_mut(&self) -> BorrowMut<'_, ValueMap> {
        self.data.borrow_mut()
    }

    /// Provides a reference to the KMap's meta map
    ///
    /// This is returned as a reference to the meta map's PtrMut to allow for cloning.
    pub fn meta_map(&self) -> &OptPtrMut<MetaMap> {
        &self.meta
    }

    /// Sets the KMap's meta map
    ///
    /// Note that this change isn't shared with maps that share the same data.
    pub fn set_meta_map(&mut self, meta: OptPtrMut<MetaMap>) {
        self.meta = meta;
    }

    /// Returns true if the meta map contains an entry with the given key
    pub fn contains_meta_key(&self, key: &MetaKey) -> bool {
        self.meta
            .as_ref()
            .is_some_and(|meta| meta.borrow().contains_key(key))
    }

    /// Returns a clone of the data value corresponding to the given key
    pub fn get<K>(&self, key: &K) -> Option<KValue>
    where
        K: Hash + Equivalent<ValueKey> + ?Sized,
    {
        self.data.borrow().get(key).cloned()
    }

    /// Returns a clone of the meta value corresponding to the given key
    pub fn get_meta_value(&self, key: &MetaKey) -> Option<KValue> {
        self.meta
            .as_ref()
            .and_then(|meta| meta.borrow().get(key).cloned())
    }

    /// Insert an entry into the KMap's data
    pub fn insert(&self, key: impl Into<ValueKey>, value: impl Into<KValue>) {
        self.data_mut().insert(key.into(), value.into());
    }

    /// Remove an entry from KMap's data
    ///
    /// If a matching entry existed in the map then its value is returned.
    ///
    /// The order of entries in the map is preserved.
    pub fn remove(&self, key: impl Into<ValueKey>) -> Option<KValue> {
        self.data_mut().shift_remove(&key.into())
    }

    /// Removes a nested entry at the given `.` separated path
    ///
    /// If a matching entry existed in the map then its value is returned.
    ///
    /// The order of entries in the map is preserved.
    pub fn remove_path(&self, path: &str) -> Option<KValue> {
        if let Some((node, rest)) = path.split_once(".") {
            self.get(node)
                .and_then(|child| match child {
                    KValue::Map(map) => Some(map),
                    _ => None,
                })
                .and_then(|nested| nested.remove_path(rest))
        } else {
            self.remove(path)
        }
    }

    /// Inserts a value into the meta map, initializing the meta map if it doesn't yet exist
    pub fn insert_meta(&mut self, key: MetaKey, value: KValue) {
        self.meta
            .get_or_insert_with(Default::default)
            .borrow_mut()
            .insert(key, value);
    }

    /// Adds a function to the KMap's data map
    pub fn add_fn(&self, id: &str, f: impl KotoFunction) {
        self.insert(id, KValue::NativeFunction(KNativeFunction::new(f)));
    }

    /// Returns the number of entries in the KMap's data map
    ///
    /// Note that this doesn't include entries in the meta map.
    pub fn len(&self) -> usize {
        self.data().len()
    }

    /// Returns true if the KMap's data map contains no entries
    ///
    /// Note that this doesn't take entries in the meta map into account.
    pub fn is_empty(&self) -> bool {
        self.data().is_empty()
    }

    /// Removes all contents from the data map, and removes the meta map
    pub fn clear(&mut self) {
        self.data_mut().clear();
        self.meta = OptPtrMut::NONE;
    }

    /// Returns true if the provided KMap occupies the same memory address
    pub fn is_same_instance(&self, other: &Self) -> bool {
        PtrMut::ptr_eq(&self.data, &other.data)
    }

    /// If present, returns the @type meta value as a [KString], recursively going up the @base chain.
    pub fn meta_type(&self) -> Option<KString> {
        use KValue::*;

        match self.get_meta_value(&MetaKey::Type) {
            Some(Str(s)) => Some(s),
            Some(_) => Some("Error: expected string as result of @type".into()),
            None => match self.get_meta_value(&MetaKey::Base) {
                Some(Map(base)) => base.meta_type(),
                _ => None,
            },
        }
    }

    /// Renders the map to the provided display context
    pub fn display(&self, ctx: &mut DisplayContext) -> Result<()> {
        if self.contains_meta_key(&UnaryOp::Display.into()) {
            let mut vm = ctx
                .vm()
                .ok_or_else(|| Error::from("missing VM in map display op"))?
                .spawn_shared_vm();
            match vm.run_unary_op(UnaryOp::Display, self.clone().into())? {
                KValue::Str(display_result) => {
                    ctx.append(display_result);
                }
                unexpected => return unexpected_type("String as @display result", &unexpected),
            }
        } else {
            if let Some(meta_type) = self.meta_type() {
                ctx.append(meta_type);
                ctx.append(' ');
            }

            ctx.append('{');

            let id = PtrMut::address(&self.data);

            if ctx.is_in_parents(id) {
                ctx.append("...");
            } else {
                ctx.push_container(id);

                for (i, (key, value)) in self.data().iter().enumerate() {
                    if i > 0 {
                        ctx.append(", ");
                    }

                    let mut key_ctx = DisplayContext::default();
                    key.value().display(&mut key_ctx)?;
                    ctx.append(key_ctx.result());
                    ctx.append(": ");

                    value.display(ctx)?;
                }

                ctx.pop_container();
            }

            ctx.append('}');
        }

        Ok(())
    }
}

impl From<ValueMap> for KMap {
    fn from(value: ValueMap) -> Self {
        KMap::with_data(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_and_remove_with_string() {
        let m = KMap::default();

        assert!(m.get("test").is_none());
        m.insert("test", KValue::Null);
        assert!(m.get("test").is_some());
        assert!(matches!(m.remove("test"), Some(KValue::Null)));
        assert!(m.get("test").is_none());
    }

    #[test]
    fn remove_path() {
        let b = KMap::default();
        b.insert("c", KValue::Null);
        b.insert("d", KValue::Null);

        let a = KMap::default();
        a.insert("b", b.clone());

        let x = KMap::default();
        x.insert("a", a);

        x.remove_path("a.b.c");

        // `b` should now have had it's `c` entry removed
        assert!(b.get("c").is_none());
        assert!(b.get("d").is_some());
    }
}
