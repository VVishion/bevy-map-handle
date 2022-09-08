## Bevy Map Handles

Map `Handle<T>` to `Handle<U>`.

All handles pointing to asset *a* of type `T` will point to asset *b* of type `U` when mapped.

```
let mapped = match handle.map_weak::<U>() {
    Err(_) => panic!("asset is pending"),
    Ok(mapped) => mapped,
};
```

Make it strong with `Assets<U>` if desired:

```
mapped.make_strong(assets);
```