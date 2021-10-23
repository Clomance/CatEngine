# ![logo](https://github.com/Clomance/CatEngine/raw/master/basement/logo_400x400.png)

# OpenGL and Windows API wrappers. [![Crate badge]][crates.io] [![Doc badge]][doc.rs]

[Crate badge]:https://img.shields.io/crates/v/cat_engine_basement.svg
[crates.io]:https://crates.io/crates/cat_engine_basement

[Doc badge]:https://img.shields.io/badge/documentation-doc.rs-green
[doc.rs]:https://docs.rs/cat_engine_basement/


There are some levels of wrapping:
 - core - just wrappers (`WinCore` and `GLCore`)
```rust
    let mut id:u32=MaybeUninit::uninit().assume_init();
    GLCore.buffer.generate_one(&mut id);

    GLCore.buffer.bind(BufferTarget::ArrayBuffer,id);

    GLCore.buffer.rewrite(BufferTarget::ArrayBuffer,1024,core::ptr::null(),BufferUsage::DynamicDraw);
```

 - level [ i ] - describes system objects and their relationships (only for `GLCore`), the higher level the more specific structures
```rust
    // level0
    let buffer:Buffer<i32>=Buffer::generate();
    buffer.bind(BufferTarget::ArrayBuffer).unwrap();
    Buffer::rewrite(BufferTarget::ArrayBuffer,&[0u32;10],BufferUsage::DynamicDraw).unwrap();

    // level1
    let index_buffer=IndexBuffer::new(&[0u32;10],BufferUsage::DynamicDraw).unwrap();
```
 - high-level - this is high level interface (only for `windows` module)
 ```rust
    let class_attributes=WindowClassAttributes::new("WNDCLASS");
    let window_class=WindowClass::new(class_attributes).unwrap();
 ```


Compatible with OpenGL 3.3 and greater.

#### Dependencies
 - Windows API - [winapi](https://github.com/retep998/winapi-rs)
 - Images - [image](https://github.com/image-rs/image)


## [Release notes](RELEASE-NOTES.MD)

## [Pre-release](https://github.com/Clomance/CatEngine/tree/pre-release)