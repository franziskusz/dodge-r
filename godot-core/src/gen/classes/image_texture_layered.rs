#![doc = "Sidecar module for class [`ImageTextureLayered`][crate::engine::ImageTextureLayered].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ImageTextureLayered` enums](https://docs.godotengine.org/en/stable/classes/class_imagetexturelayered.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::builtin::meta::{
    ClassName, PtrcallReturnUnit, PtrcallReturnT, PtrcallReturnOptionGdT, PtrcallSignatureTuple, VarcallSignatureTuple
};
use crate::engine::native::*;
use crate::engine::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::engine::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `ImageTextureLayered.`\n\nInherits [`TextureLayered`][crate::engine::TextureLayered].\n\nRelated symbols:\n\n* [`IImageTextureLayered`][crate::engine::IImageTextureLayered]: virtual methods\n\n\nSee also [Godot docs for `ImageTextureLayered`](https://docs.godotengine.org/en/stable/classes/class_imagetexturelayered.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ImageTextureLayered {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ImageTextureLayered`][crate::engine::ImageTextureLayered].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ImageTextureLayered` methods](https://docs.godotengine.org/en/stable/classes/class_imagetexturelayered.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IImageTextureLayered: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `#[base]` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::engine::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented !()
        }
        fn get_format(&self,) -> crate::engine::image::Format {
            unimplemented !()
        }
        fn get_layered_type(&self,) -> u32 {
            unimplemented !()
        }
        fn get_width(&self,) -> i32 {
            unimplemented !()
        }
        fn get_height(&self,) -> i32 {
            unimplemented !()
        }
        fn get_layers(&self,) -> i32 {
            unimplemented !()
        }
        fn has_mipmaps(&self,) -> bool {
            unimplemented !()
        }
        fn get_layer_data(&self, layer_index: i32,) -> Option < Gd < crate::engine::Image > > {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl ImageTextureLayered {
        pub fn create_from_images(&mut self, images: Array < Gd < crate::engine::Image > >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Array < Gd < crate::engine::Image > >);
            let args = (images,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_from_images", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_layer(&mut self, image: Gd < crate::engine::Image >, layer: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Image >, i32);
            let args = (image, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "update_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for ImageTextureLayered {
        type Base = crate::engine::TextureLayered;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ImageTextureLayered\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ImageTextureLayered {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ImageTextureLayered {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::TextureLayered > for ImageTextureLayered {
        
    }
    impl crate::obj::Inherits < crate::engine::Texture > for ImageTextureLayered {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for ImageTextureLayered {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ImageTextureLayered {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ImageTextureLayered {
        
    }
    impl crate::obj::ExportableObject for ImageTextureLayered {
        
    }
    impl std::ops::Deref for ImageTextureLayered {
        type Target = crate::engine::TextureLayered;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ImageTextureLayered {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ImageTextureLayered {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ImageTextureLayered > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::TextureLayered > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Texture > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Resource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}