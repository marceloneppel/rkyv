//! Low-level checked APIs.
//!
//! These APIs do not support shared pointers.

use bytecheck::CheckBytes;
use rancor::{Source, Strategy};

use crate::{
    api::{
        access_pos_unchecked_mut, access_pos_with_context, access_with_context,
        check_pos_with_context, deserialize_using, root_position,
    },
    de::pooling::Unpool,
    seal::Seal,
    validation::{archive::ArchiveValidator, Validator},
    Archive, Deserialize, Portable,
};

/// A low-level validator.
///
/// This is part of the [low-level API](crate::api::low).
pub type LowValidator<'a, E> = Strategy<Validator<ArchiveValidator<'a>, ()>, E>;

fn validator(bytes: &[u8]) -> Validator<ArchiveValidator<'_>, ()> {
    Validator::new(ArchiveValidator::new(bytes), ())
}

/// Accesses an archived value from the given byte slice at the given position
/// after checking its validity.
///
/// This is a safe alternative to
/// [`access_pos_unchecked`](crate::api::access_pos_unchecked) and is part of
/// the [low-level API](crate::api::low).
pub fn access_pos<T, E>(bytes: &[u8], pos: usize) -> Result<&T, E>
where
    T: Portable + for<'a> CheckBytes<LowValidator<'a, E>>,
    E: Source,
{
    access_pos_with_context::<_, _, E>(bytes, pos, &mut validator(bytes))
}

/// Accesses an archived value from the given byte slice by calculating the root
/// position after checking its validity.
///
/// This is a safe alternative to
/// [`access_unchecked`](crate::api::access_unchecked) and is part of the
/// [low-level API](crate::api::low).
pub fn access<T, E>(bytes: &[u8]) -> Result<&T, E>
where
    T: Portable + for<'a> CheckBytes<LowValidator<'a, E>>,
    E: Source,
{
    access_with_context::<_, _, E>(bytes, &mut validator(bytes))
}

/// Mutably accesses an archived value from the given byte slice at the given
/// position after checking its validity.
///
/// This is a safe alternative to
/// [`access_pos_unchecked`](crate::api::access_pos_unchecked) and is part of
/// the [low-level API](crate::api::low).
pub fn access_pos_mut<T, E>(
    bytes: &mut [u8],
    pos: usize,
) -> Result<Seal<'_, T>, E>
where
    T: Portable + for<'a> CheckBytes<LowValidator<'a, E>>,
    E: Source,
{
    let mut context = validator(bytes);
    check_pos_with_context::<T, _, E>(bytes, pos, &mut context)?;
    unsafe { Ok(access_pos_unchecked_mut::<T>(bytes, pos)) }
}

/// Mutably accesses an archived value from the given byte slice by calculating
/// the root position after checking its validity.
///
/// This is a safe alternative to
/// [`access_unchecked`](crate::api::access_unchecked) and is part of the
/// [low-level API](crate::api::low).
pub fn access_mut<T, E>(bytes: &mut [u8]) -> Result<Seal<'_, T>, E>
where
    T: Portable + for<'a> CheckBytes<LowValidator<'a, E>>,
    E: Source,
{
    let mut context = validator(bytes);
    let pos = root_position::<T>(bytes.len());
    check_pos_with_context::<T, _, E>(bytes, pos, &mut context)?;
    unsafe { Ok(access_pos_unchecked_mut::<T>(bytes, pos)) }
}

/// Checks and deserializes a value from the given bytes.
///
/// This is a safe alternative to
/// [`from_bytes_unchecked`](crate::api::low::from_bytes_unchecked) and is part
/// of the [low-level API](crate::api::low).
pub fn from_bytes<T, E>(bytes: &[u8]) -> Result<T, E>
where
    T: Archive,
    T::Archived: for<'a> CheckBytes<LowValidator<'a, E>>
        + Deserialize<T, Strategy<Unpool, E>>,
    E: Source,
{
    deserialize_using(access::<T::Archived, E>(bytes)?, &mut Unpool)
}
