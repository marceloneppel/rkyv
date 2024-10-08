//! APIs for environments where allocations cannot be made.
//!
//! These APIs require user-provided writers and allocators, and do not support
//! shared pointers.

#[cfg(feature = "bytecheck")]
mod checked;

use rancor::Strategy;

#[cfg(feature = "bytecheck")]
pub use self::checked::*;
use crate::{
    access_unchecked,
    api::{deserialize_using, serialize_using},
    ser::{Allocator, Serializer, Writer},
    Archive, Deserialize, Serialize,
};

/// A general-purpose serializer suitable for environments where allocations
/// cannot be made.
///
/// This is part of the [low-level API](crate::api::low).
pub type LowSerializer<'a, W, A, E> = Strategy<Serializer<W, A, ()>, E>;

/// A general-purpose deserializer suitable for environments where allocations
/// cannot be made.
///
/// This is part of the [low-level API](crate::api::low).
pub type LowDeserializer<E> = Strategy<(), E>;

/// Serializes the given value and writes the bytes to the given `writer`, using
/// the given allocator.
///
/// This is part of the [low-level API](crate::api::low).
pub fn to_bytes_in_with_alloc<'a, W, A, E>(
    value: &impl Serialize<LowSerializer<'a, W, A, E>>,
    writer: W,
    alloc: A,
) -> Result<W, E>
where
    W: Writer<E>,
    A: Allocator<E>,
    E: rancor::Source,
{
    let mut serializer = Serializer::new(writer, alloc, ());
    serialize_using(value, &mut serializer)?;
    Ok(serializer.into_writer())
}

/// Deserializes a value from the given bytes.
///
/// This function does not check that the data is valid. Use [`from_bytes`] to
/// validate the data instead.
///
/// This is part of the [low-level API](crate::api::low).
///
/// # Safety
///
/// The given bytes must pass validation when passed to [`from_bytes`].
pub unsafe fn from_bytes_unchecked<T, E>(bytes: &[u8]) -> Result<T, E>
where
    T: Archive,
    T::Archived: Deserialize<T, LowDeserializer<E>>,
{
    // SAFETY: The caller has guaranteed that a valid `T` is located at the root
    // position in the byte slice.
    let archived = unsafe { access_unchecked::<T::Archived>(bytes) };
    deserialize(archived)
}

/// Deserializes a value from the given archived value.
///
/// This is part of the [low-level API](crate::api::low).
pub fn deserialize<T, E>(
    value: &impl Deserialize<T, LowDeserializer<E>>,
) -> Result<T, E> {
    deserialize_using(value, &mut ())
}
