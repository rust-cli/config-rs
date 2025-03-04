/// The backing store for [`Config`][crate::Config]
pub type Map<K, V> = InternalMap<K, V>;

#[cfg(not(feature = "preserve_order"))]
type InternalMap<K, V> = std::collections::HashMap<K, V>;
#[cfg(feature = "preserve_order")]
type InternalMap<K, V> = indexmap::IndexMap<K, V>;
