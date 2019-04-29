//! The `TypedArray`/`MemoryView` WebAssembly classes.

macro_rules! memory_view {
    ($mod_name:ident over $wasm_type:ty) => {
        pub mod $mod_name {
            use lazy_static::lazy_static;
            use rutie::{class, methods, wrappable_struct, Fixnum, Integer, NilClass, Object};
            use std::{mem::size_of, rc::Rc};
            use wasmer_runtime as runtime;

            pub struct MemoryView {
                memory: Rc<runtime::memory::Memory>,
                offset: usize,
            }

            impl MemoryView {
                pub fn new(memory: Rc<runtime::Memory>, offset: usize) -> Self {
                    Self { memory, offset }
                }

                pub fn len(&self) -> usize {
                    self.memory.view::<$wasm_type>()[self.offset..].len() / size_of::<$wasm_type>()
                }

                pub fn set(&self, index: isize, value: $wasm_type) -> Result<(), &str> {
                    let offset = self.offset;
                    let view = self.memory.view::<$wasm_type>();

                    if index < 0 {
                        return Err("foo");
                    }

                    let index = index as usize;

                    if view.len() <= offset + index {
                        Err("bar")
                    } else {
                        view[offset + index].set(value);

                        Ok(())
                    }
                }

                pub fn get(&self, index: isize) -> Result<$wasm_type, &str> {
                    let offset = self.offset;
                    let view = self.memory.view::<$wasm_type>();

                    if index < 0 {
                        return Err("foo");
                    }

                    let index = index as usize;

                    if view.len() <= offset + index {
                        Err("bar")
                    } else {
                        Ok(view[offset + index].get())
                    }
                }
            }

            wrappable_struct!(MemoryView, MemoryViewWrapper, MEMORY_VIEW_WRAPPER);

            class!(RubyMemoryView);

            #[rustfmt::skip]
            methods!(
                RubyMemoryView,
                itself,

                // Glue code to call the `TypedArray.length` method.
                fn ruby_memory_view_length() -> Fixnum {
                    Fixnum::new(itself.get_data(&*MEMORY_VIEW_WRAPPER).len() as i64)
                }

                // Glue code to call the `TypedArray.set` method.
                fn ruby_memory_view_set(index: Integer, value: Integer) -> NilClass {
                    let memory_view = itself.get_data(&*MEMORY_VIEW_WRAPPER);
                    memory_view.set(index.unwrap().to_i32() as isize, value.unwrap().to_i32() as $wasm_type).unwrap();

                    NilClass::new()
                }

                // Glue code to call the `TypedArray.get` method.
                fn ruby_memory_view_get(index: Integer) -> Fixnum {
                    let memory_view = itself.get_data(&*MEMORY_VIEW_WRAPPER);

                    Fixnum::new(memory_view.get(index.unwrap().to_i32() as isize).unwrap() as i64)
                }
            );
        }
    }
}

memory_view!(uint8array over u8);
memory_view!(int8array over i8);
memory_view!(uint16array over u16);
memory_view!(int16array over i16);
memory_view!(uint32array over u32);
memory_view!(int32array over i32);
