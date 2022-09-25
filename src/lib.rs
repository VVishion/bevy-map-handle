use bevy::{
    asset::{Asset, HandleId},
    prelude::Handle,
};

#[derive(Debug)]
pub enum MapHandleError {
    /// The asset is still pending and has no id assigned.
    PendingAsset,
}

/// Implement methods for mapping [`Handle<T>`] to [`Handle<U>`] for [`Handle<T>`].
/// The asset id is preserved so that all handles pointing to asset *a* of type `T` will point to asset *b* of type `U` when mapped.
/// This is also true for mapping from `U` to `T`.
pub trait MapHandle {
    fn map_weak<T>(&self) -> Result<Handle<T>, MapHandleError>
    where
        T: Asset;
}

impl<T> MapHandle for Handle<T>
where
    T: Asset,
{
    /// Map [`Handle<T>`] to [`Handle<U>`] weak.
    /// The asset id is preserved so that all handles pointing to asset *a* of type `T` will point to asset *b* of type `U` when mapped.
    /// This is also true for mapping from `U` to `T`.
    /// Will return Err [`MapHandleError::PendingAsset`] when trying to map handles of pending assets.
    /// make it strong with `Assets<U>` if desired:
    /// ```
    /// mapped.make_strong(assets);
    /// ```
    fn map_weak<U>(&self) -> Result<Handle<U>, MapHandleError>
    where
        U: Asset,
    {
        match self.id {
            HandleId::AssetPathId(_) => Err(MapHandleError::PendingAsset),
            HandleId::Id(_, id) => {
                let handle_id = HandleId::new(U::TYPE_UUID, id.clone());
                Ok(Handle::<U>::weak(handle_id))
            }
        }
    }
}
