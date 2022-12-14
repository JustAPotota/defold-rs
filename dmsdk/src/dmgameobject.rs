//! Functions for manipulating game objects.

use crate::{dmvmath, ffi::dmGameObject};

/// Game object register.
pub type Register = dmGameObject::HRegister;
/// Game object instance.
pub type Instance = dmGameObject::HInstance;

const UNNAMED_IDENTIFIER: u64 = 12415623704795185700;

#[allow(missing_docs)]
pub enum Error {
    OutOfResources,
    AlreadyRegistered,
    IdentifierInUse,
    IdentifierAlreadySet,
    ComponentNotFound,
    MaximumHierarchicalDepth,
    InvalidOperation,
    ResourceTypeNotFound,
    BufferOverflow,
    Unknown,
}

impl From<i32> for Error {
    fn from(x: i32) -> Self {
        match x {
            -1 => Self::OutOfResources,
            -2 => Self::AlreadyRegistered,
            -3 => Self::IdentifierInUse,
            -4 => Self::IdentifierAlreadySet,
            -5 => Self::ComponentNotFound,
            -6 => Self::MaximumHierarchicalDepth,
            -7 => Self::InvalidOperation,
            -8 => Self::ResourceTypeNotFound,
            -9 => Self::BufferOverflow,
            _ => Self::Unknown,
        }
    }
}

/// [`Result`](core::result::Result) alias with an error type of [`Error`].
pub type Result<T> = core::result::Result<T, Error>;

/// Returns the ID of a game object if it has one.
///
/// # Safety
///
/// This function is safe as long as `instance` points to a valid game object.
pub unsafe fn get_identifier(instance: Instance) -> Option<u64> {
    let hash = dmGameObject::GetIdentifier(instance);
    if hash == UNNAMED_IDENTIFIER {
        None
    } else {
        Some(hash)
    }
}

/// Returns the position of a game object.
///
/// # Safety
///
/// This function is safe as long as `instance` points to a valid game object.
pub unsafe fn get_position(instance: Instance) -> dmvmath::Point3 {
    dmGameObject::GetPosition(instance).into()
}

/// Returns the rotation of a game object.
///
/// # Safety
///
/// This function is safe as long as `instance` points to a valid game object.
pub unsafe fn get_rotation(instance: Instance) -> dmvmath::Quat {
    dmGameObject::GetRotation(instance).into()
}

/// Returns the scale of a game object.
///
/// # Safety
///
/// This function is safe as long as `instance` points to a valid game object.
pub unsafe fn get_scale(instance: Instance) -> dmvmath::Vector3 {
    dmGameObject::GetScale(instance).into()
}

/// Sets the position of a game object.
///
/// # Safety
///
/// This function is safe as long as `instance` points to a valid game object.
pub unsafe fn set_position(instance: Instance, position: dmvmath::Point3) {
    dmGameObject::SetPosition(instance, position.into())
}
