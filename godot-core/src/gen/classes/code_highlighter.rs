#![doc = "Sidecar module for class [`CodeHighlighter`][crate::engine::CodeHighlighter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CodeHighlighter` enums](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CodeHighlighter.`\n\nInherits [`SyntaxHighlighter`][crate::engine::SyntaxHighlighter].\n\nRelated symbols:\n\n* [`code_highlighter`][crate::engine::code_highlighter]: sidecar module with related enum/flag types\n* [`ICodeHighlighter`][crate::engine::ICodeHighlighter]: virtual methods\n\n\nSee also [Godot docs for `CodeHighlighter`](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CodeHighlighter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CodeHighlighter`][crate::engine::CodeHighlighter].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CodeHighlighter` methods](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICodeHighlighter: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_line_syntax_highlighting(&self, line: i32,) -> Dictionary {
            unimplemented !()
        }
        fn clear_highlighting_cache(&mut self,) {
            unimplemented !()
        }
        fn update_cache(&mut self,) {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl CodeHighlighter {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn add_keyword_color(&mut self, keyword: GString, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Color);
            let args = (keyword, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_keyword_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_keyword_color(&mut self, keyword: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (keyword,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_keyword_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_keyword_color(&self, keyword: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (keyword,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_keyword_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_keyword_color(&self, keyword: GString,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, GString);
            let args = (keyword,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_keyword_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_keyword_colors(&mut self, keywords: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Dictionary);
            let args = (keywords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_keyword_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_keyword_colors(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_keyword_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_keyword_colors(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_keyword_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_member_keyword_color(&mut self, member_keyword: GString, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, Color);
            let args = (member_keyword, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_member_keyword_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_member_keyword_color(&mut self, member_keyword: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (member_keyword,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_member_keyword_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_member_keyword_color(&self, member_keyword: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (member_keyword,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_member_keyword_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_member_keyword_color(&self, member_keyword: GString,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, GString);
            let args = (member_keyword,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_member_keyword_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_member_keyword_colors(&mut self, member_keyword: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Dictionary);
            let args = (member_keyword,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_member_keyword_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_member_keyword_colors(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_member_keyword_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_member_keyword_colors(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_member_keyword_colors", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_color_region_full(&mut self, start_key: GString, end_key: GString, color: Color, line_only: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, Color, bool);
            let args = (start_key, end_key, color, line_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_color_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_color_region(&mut self, start_key: GString, end_key: GString, color: Color,) {
            self.add_color_region_ex(start_key, end_key, color,) . done()
        }
        #[inline]
        pub fn add_color_region_ex(&mut self, start_key: GString, end_key: GString, color: Color,) -> ExAddColorRegion < '_ > {
            ExAddColorRegion::new(self, start_key, end_key, color,)
        }
        pub fn remove_color_region(&mut self, start_key: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (start_key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_color_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_color_region(&self, start_key: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (start_key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_color_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_regions(&mut self, color_regions: Dictionary,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Dictionary);
            let args = (color_regions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color_regions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_color_regions(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2009usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_color_regions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_regions(&self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2010usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_regions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_function_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_function_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_function_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_function_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_number_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_number_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_number_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_number_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_symbol_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_symbol_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_symbol_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2016usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_symbol_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_member_variable_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2017usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_member_variable_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_member_variable_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2018usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_member_variable_color", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CodeHighlighter {
        type Base = crate::engine::SyntaxHighlighter;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"CodeHighlighter\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CodeHighlighter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for CodeHighlighter {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::SyntaxHighlighter > for CodeHighlighter {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for CodeHighlighter {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for CodeHighlighter {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for CodeHighlighter {
        
    }
    impl crate::obj::ExportableObject for CodeHighlighter {
        
    }
    impl crate::obj::cap::GodotDefault for CodeHighlighter {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CodeHighlighter {
        type Target = crate::engine::SyntaxHighlighter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CodeHighlighter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_CodeHighlighter {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::CodeHighlighter > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::SyntaxHighlighter > for $Class {
                
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
#[doc = "Default-param extender for [`CodeHighlighter::add_color_region_ex`][super::CodeHighlighter::add_color_region_ex]."]
#[must_use]
pub struct ExAddColorRegion < 'a > {
    surround_object: &'a mut re_export::CodeHighlighter, start_key: GString, end_key: GString, color: Color, line_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddColorRegion < 'a > {
    fn new(surround_object: &'a mut re_export::CodeHighlighter, start_key: GString, end_key: GString, color: Color,) -> Self {
        Self {
            surround_object, start_key, end_key, color, line_only: false,
        }
    }
    #[inline]
    pub fn line_only(self, value: bool) -> Self {
        Self {
            line_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::CodeHighlighter::add_color_region_full(self.surround_object, self.start_key, self.end_key, self.color, self.line_only,)
    }
}