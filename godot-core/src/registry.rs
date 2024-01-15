/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::meta::ClassName;
use crate::builtin::StringName;
use crate::init::InitLevel;
use crate::obj::*;
use crate::out;
use crate::private::as_storage;
use crate::storage::InstanceStorage;
use godot_ffi as sys;

use sys::{interface_fn, Global, GlobalGuard, GlobalLockError};

use std::any::Any;
use std::collections::HashMap;
use std::{fmt, ptr};

// Needed for class unregistering. The variable is populated during class registering. There is no actual concurrency here, because Godot
// calls register/unregister in the main thread. Mutex is just casual way to ensure safety in this non-performance-critical path.
// Note that we panic on concurrent access instead of blocking (fail-fast approach). If that happens, most likely something changed on Godot
// side and analysis required to adopt these changes.
static LOADED_CLASSES: Global<HashMap<InitLevel, Vec<ClassName>>> = Global::default();

// TODO(bromeon): some information coming from the proc-macro API is deferred through PluginComponent, while others is directly
// translated to code. Consider moving more code to the PluginComponent, which allows for more dynamic registration and will
// be easier for a future builder API.

/// Piece of information that is gathered by the self-registration ("plugin") system.
#[derive(Debug)]
pub struct ClassPlugin {
    pub class_name: ClassName,
    pub component: PluginComponent,

    // Init-level is per ClassPlugin and not per PluginComponent, because all components of all classes are mixed together in one
    // huge linker list. There is no per-class aggregation going on, so this allows to easily filter relevant classes.
    pub init_level: InitLevel,
}

/// Type-erased function object, holding a `register_class` function.
#[derive(Copy, Clone)]
pub struct ErasedRegisterFn {
    // Wrapper needed because Debug can't be derived on function pointers with reference parameters, so this won't work:
    // pub type ErasedRegisterFn = fn(&mut dyn std::any::Any);
    // (see https://stackoverflow.com/q/53380040)
    pub raw: fn(&mut dyn Any),
}

impl fmt::Debug for ErasedRegisterFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:0>16x}", self.raw as usize)
    }
}

/// Represents the data part of a [`ClassPlugin`] instance.
#[derive(Clone, Debug)]
pub enum PluginComponent {
    /// Class definition itself, must always be available.
    ClassDef {
        base_class_name: ClassName,

        /// Godot low-level `create` function, wired up to library-generated `init`.
        generated_create_fn: Option<
            unsafe extern "C" fn(
                _class_userdata: *mut std::ffi::c_void, //
            ) -> sys::GDExtensionObjectPtr,
        >,

        generated_recreate_fn: Option<
            unsafe extern "C" fn(
                p_class_userdata: *mut std::ffi::c_void,
                p_object: sys::GDExtensionObjectPtr,
            ) -> sys::GDExtensionClassInstancePtr,
        >,

        /// Callback to library-generated function which registers properties in the `struct` definition.
        register_properties_fn: ErasedRegisterFn,

        free_fn: unsafe extern "C" fn(
            _class_user_data: *mut std::ffi::c_void,
            instance: sys::GDExtensionClassInstancePtr,
        ),

        /// Calls `__before_ready()`, if there is at least one `OnReady` field. Used if there is no `#[godot_api] impl` block
        /// overriding ready.
        default_get_virtual_fn: Option<
            unsafe extern "C" fn(
                p_userdata: *mut std::os::raw::c_void,
                p_name: sys::GDExtensionConstStringNamePtr,
            ) -> sys::GDExtensionClassCallVirtual,
        >,
    },

    /// Collected from `#[godot_api] impl MyClass`
    UserMethodBinds {
        /// Callback to library-generated function which registers functions and constants in the `impl` block.
        ///
        /// Always present since that's the entire point of this `impl` block.
        register_methods_constants_fn: ErasedRegisterFn,
    },

    /// Collected from `#[godot_api] impl GodotExt for MyClass`
    UserVirtuals {
        /// Callback to user-defined `register_class` function.
        user_register_fn: Option<ErasedRegisterFn>,

        /// Godot low-level `create` function, wired up to the user's `init`.
        user_create_fn: Option<
            unsafe extern "C" fn(
                _class_userdata: *mut std::ffi::c_void, //
            ) -> sys::GDExtensionObjectPtr,
        >,

        user_recreate_fn: Option<
            unsafe extern "C" fn(
                p_class_userdata: *mut ::std::os::raw::c_void,
                p_object: sys::GDExtensionObjectPtr,
            ) -> sys::GDExtensionClassInstancePtr,
        >,

        /// User-defined `to_string` function.
        user_to_string_fn: Option<
            unsafe extern "C" fn(
                p_instance: sys::GDExtensionClassInstancePtr,
                r_is_valid: *mut sys::GDExtensionBool,
                r_out: sys::GDExtensionStringPtr,
            ),
        >,

        /// User-defined `on_notification` function.
        #[cfg(before_api = "4.2")]
        user_on_notification_fn: Option<
            unsafe extern "C" fn(
                p_instance: sys::GDExtensionClassInstancePtr, //
                p_what: i32,
            ),
        >,
        #[cfg(since_api = "4.2")]
        user_on_notification_fn: Option<
            unsafe extern "C" fn(
                p_instance: sys::GDExtensionClassInstancePtr, //
                p_what: i32,
                p_reversed: sys::GDExtensionBool,
            ),
        >,

        /// Callback for other virtuals.
        get_virtual_fn: unsafe extern "C" fn(
            p_userdata: *mut std::os::raw::c_void,
            p_name: sys::GDExtensionConstStringNamePtr,
        ) -> sys::GDExtensionClassCallVirtual,
    },

    #[cfg(since_api = "4.1")]
    EditorPlugin,
    #[cfg(since_api = "4.2")]
    Unexposed,
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct ClassRegistrationInfo {
    class_name: ClassName,
    parent_class_name: Option<ClassName>,
    // Following functions are stored separately, since their order matters.
    register_methods_constants_fn: Option<ErasedRegisterFn>,
    register_properties_fn: Option<ErasedRegisterFn>,
    user_register_fn: Option<ErasedRegisterFn>,
    default_virtual_fn: sys::GDExtensionClassGetVirtual, // Option (set if there is at least one OnReady field)
    user_virtual_fn: sys::GDExtensionClassGetVirtual, // Option (set if there is a `#[godot_api] impl I*`)

    /// Godot low-level class creation parameters.
    #[cfg(before_api = "4.2")]
    godot_params: sys::GDExtensionClassCreationInfo,
    #[cfg(since_api = "4.2")]
    godot_params: sys::GDExtensionClassCreationInfo2,
    #[allow(dead_code)] // Currently unused; may be useful for diagnostics in the future.
    init_level: InitLevel,
    is_editor_plugin: bool,
}

/// Registers a class with static type information.
pub fn register_class<
    T: cap::GodotDefault
        + cap::ImplementsGodotVirtual
        + cap::GodotToString
        + cap::GodotNotification
        + cap::GodotRegisterClass
        + GodotClass,
>() {
    // TODO: provide overloads with only some trait impls

    out!("Manually register class {}", std::any::type_name::<T>());

    #[cfg(before_api = "4.2")]
    let godot_params = sys::GDExtensionClassCreationInfo {
        to_string_func: Some(callbacks::to_string::<T>),
        notification_func: Some(callbacks::on_notification::<T>),
        reference_func: Some(callbacks::reference::<T>),
        unreference_func: Some(callbacks::unreference::<T>),
        create_instance_func: Some(callbacks::create::<T>),
        free_instance_func: Some(callbacks::free::<T>),
        get_virtual_func: Some(callbacks::get_virtual::<T>),
        get_rid_func: None,
        class_userdata: ptr::null_mut(), // will be passed to create fn, but global per class
        ..default_creation_info()
    };
    #[cfg(since_api = "4.2")]
    let godot_params = sys::GDExtensionClassCreationInfo2 {
        to_string_func: Some(callbacks::to_string::<T>),
        notification_func: Some(callbacks::on_notification::<T>),
        reference_func: Some(callbacks::reference::<T>),
        unreference_func: Some(callbacks::unreference::<T>),
        create_instance_func: Some(callbacks::create::<T>),
        recreate_instance_func: Some(callbacks::recreate::<T>),
        free_instance_func: Some(callbacks::free::<T>),
        get_virtual_func: Some(callbacks::get_virtual::<T>),
        get_rid_func: None,
        class_userdata: ptr::null_mut(), // will be passed to create fn, but global per class
        ..default_creation_info()
    };

    assert!(
        !T::class_name().is_empty(),
        "cannot register () or unnamed class"
    );

    register_class_raw(ClassRegistrationInfo {
        class_name: T::class_name(),
        parent_class_name: Some(T::Base::class_name()),
        register_methods_constants_fn: None,
        register_properties_fn: None,
        user_register_fn: Some(ErasedRegisterFn {
            raw: callbacks::register_class_by_builder::<T>,
        }),
        user_virtual_fn: None,
        default_virtual_fn: None,
        godot_params,
        init_level: T::INIT_LEVEL,
        is_editor_plugin: false,
    });
}

/// Lets Godot know about all classes that have self-registered through the plugin system.
pub fn auto_register_classes(init_level: InitLevel) {
    out!("Auto-register classes at level `{init_level:?}`...");

    // Note: many errors are already caught by the compiler, before this runtime validation even takes place:
    // * missing #[derive(GodotClass)] or impl GodotClass for T
    // * duplicate impl GodotDefault for T
    //
    let mut map = HashMap::<ClassName, ClassRegistrationInfo>::new();

    crate::private::iterate_plugins(|elem: &ClassPlugin| {
        //out!("* Plugin: {elem:#?}");

        // Filter per ClassPlugin and not PluginComponent, because all components of all classes are mixed together in one huge list.
        if elem.init_level != init_level {
            return;
        }

        let name = elem.class_name;
        let class_info = map
            .entry(name)
            .or_insert_with(|| default_registration_info(name));

        fill_class_info(elem.component.clone(), class_info);
    });

    let mut loaded_classes_by_level = global_loaded_classes();
    for info in map.into_values() {
        out!(
            "Register class:   {} at level `{init_level:?}`",
            info.class_name
        );
        let class_name = info.class_name;
        loaded_classes_by_level
            .entry(init_level)
            .or_default()
            .push(info.class_name);

        register_class_raw(info);
        out!("Class {} loaded", class_name);
    }

    out!("All classes for level `{init_level:?}` auto-registered.");
}

pub fn unregister_classes(init_level: InitLevel) {
    let mut loaded_classes_by_level = global_loaded_classes();
    let loaded_classes_current_level = loaded_classes_by_level
        .remove(&init_level)
        .unwrap_or_default();
    out!("Unregistering classes of level {init_level:?}...");
    for class_name in loaded_classes_current_level.iter().rev() {
        unregister_class_raw(*class_name);
    }
}

fn global_loaded_classes() -> GlobalGuard<'static, HashMap<InitLevel, Vec<ClassName>>> {
    match LOADED_CLASSES.try_lock() {
        Ok(it) => it,
        Err(err) => match err {
            GlobalLockError::Poisoned {..} => panic!(
                "global lock for loaded classes poisoned; class registration or deregistration may have panicked"
            ),
            GlobalLockError::WouldBlock => panic!("unexpected concurrent access to global lock for loaded classes"),
            GlobalLockError::InitFailed => unreachable!("global lock for loaded classes not initialized"),
        },
    }
}

/// Populate `c` with all the relevant data from `component` (depending on component type).
fn fill_class_info(component: PluginComponent, c: &mut ClassRegistrationInfo) {
    // out!("|   reg (before):    {c:?}");
    // out!("|   comp:            {component:?}");
    match component {
        PluginComponent::ClassDef {
            base_class_name,
            generated_create_fn,
            generated_recreate_fn,
            register_properties_fn,
            free_fn,
            default_get_virtual_fn,
        } => {
            c.parent_class_name = Some(base_class_name);

            fill_into(
                &mut c.godot_params.create_instance_func,
                generated_create_fn,
            )
            .unwrap_or_else(|_|
                panic!(
                    "Godot class `{}` is defined multiple times in Rust; you can rename them with #[class(rename=NewName)]",
                    c.class_name,
                )
            );

            #[cfg(since_api = "4.2")]
            fill_into(
                &mut c.godot_params.recreate_instance_func,
                generated_recreate_fn,
            )
            .unwrap_or_else(|_|
                panic!(
                    "Godot class `{}` is defined multiple times in Rust; you can rename them with #[class(rename=NewName)]",
                    c.class_name,
                )
            );

            #[cfg(before_api = "4.2")]
            assert!(generated_recreate_fn.is_none()); // not used

            c.godot_params.free_instance_func = Some(free_fn);
            c.default_virtual_fn = default_get_virtual_fn;
            c.register_properties_fn = Some(register_properties_fn);
        }

        PluginComponent::UserMethodBinds {
            register_methods_constants_fn,
        } => {
            c.register_methods_constants_fn = Some(register_methods_constants_fn);
        }

        PluginComponent::UserVirtuals {
            user_register_fn,
            user_create_fn,
            user_recreate_fn,
            user_to_string_fn,
            user_on_notification_fn,
            get_virtual_fn,
        } => {
            c.user_register_fn = user_register_fn;

            // The following unwraps of fill_into() shouldn't panic, since rustc will error if there are
            // multiple `impl I{Class} for Thing` definitions.

            fill_into(&mut c.godot_params.create_instance_func, user_create_fn).unwrap();

            #[cfg(since_api = "4.2")]
            fill_into(&mut c.godot_params.recreate_instance_func, user_recreate_fn).unwrap();

            #[cfg(before_api = "4.2")]
            assert!(user_recreate_fn.is_none()); // not used

            c.godot_params.to_string_func = user_to_string_fn;
            c.godot_params.notification_func = user_on_notification_fn;
            c.user_virtual_fn = Some(get_virtual_fn);
        }
        #[cfg(since_api = "4.1")]
        PluginComponent::EditorPlugin => {
            c.is_editor_plugin = true;
        }
        #[cfg(since_api = "4.2")]
        PluginComponent::Unexposed => {
            c.godot_params.is_exposed = false as sys::GDExtensionBool;
        }
    }
    // out!("|   reg (after):     {c:?}");
    // out!();
}

/// If `src` is occupied, it moves the value into `dst`, while ensuring that no previous value is present in `dst`.
fn fill_into<T>(dst: &mut Option<T>, src: Option<T>) -> Result<(), ()> {
    match (dst, src) {
        (dst @ None, src) => *dst = src,
        (Some(_), Some(_)) => return Err(()),
        (Some(_), None) => { /* do nothing */ }
    }
    Ok(())
}

/// Registers a class with given the dynamic type information `info`.
fn register_class_raw(mut info: ClassRegistrationInfo) {
    // First register class...

    let class_name = info.class_name;
    let parent_class_name = info
        .parent_class_name
        .expect("class defined (parent_class_name)");

    // Register virtual functions -- if the user provided some via #[godot_api], take those; otherwise, use the
    // ones generated alongside #[derive(GodotClass)]. The latter can also be null, if no OnReady is provided.
    if info.godot_params.get_virtual_func.is_none() {
        info.godot_params.get_virtual_func = info.user_virtual_fn.or(info.default_virtual_fn);
    }

    unsafe {
        // Try to register class...

        #[cfg(before_api = "4.2")]
        #[allow(clippy::let_unit_value)]
        // notifies us if Godot API ever adds a return type.
        let _: () = interface_fn!(classdb_register_extension_class)(
            sys::get_library(),
            class_name.string_sys(),
            parent_class_name.string_sys(),
            ptr::addr_of!(info.godot_params),
        );

        #[cfg(since_api = "4.2")]
        #[allow(clippy::let_unit_value)]
        // notifies us if Godot API ever adds a return type.
        let _: () = interface_fn!(classdb_register_extension_class2)(
            sys::get_library(),
            class_name.string_sys(),
            parent_class_name.string_sys(),
            ptr::addr_of!(info.godot_params),
        );

        // ...then see if it worked.
        // This is necessary because the above registration does not report errors (apart from console output).
        let tag = interface_fn!(classdb_get_class_tag)(class_name.string_sys());
        assert!(
            !tag.is_null(),
            "failed to register class `{class_name}`; check preceding Godot stderr messages",
        );
    }

    // ...then custom symbols

    //let mut class_builder = crate::builder::ClassBuilder::<?>::new();
    let mut class_builder = 0; // TODO dummy argument; see callbacks

    // Order of the following registrations is crucial:
    // 1. Methods and constants.
    // 2. Properties (they may depend on get/set methods).
    // 3. User-defined registration function (intuitively, user expects their own code to run after proc-macro generated code).
    if let Some(register_fn) = info.register_methods_constants_fn {
        (register_fn.raw)(&mut class_builder);
    }

    if let Some(register_fn) = info.register_properties_fn {
        (register_fn.raw)(&mut class_builder);
    }

    if let Some(register_fn) = info.user_register_fn {
        (register_fn.raw)(&mut class_builder);
    }

    #[cfg(since_api = "4.1")]
    if info.is_editor_plugin {
        unsafe { interface_fn!(editor_add_plugin)(class_name.string_sys()) };
    }
    #[cfg(before_api = "4.1")]
    assert!(!info.is_editor_plugin);
}

fn unregister_class_raw(class_name: ClassName) {
    out!("Unregister class: {class_name}");
    unsafe {
        #[allow(clippy::let_unit_value)]
        let _: () = interface_fn!(classdb_unregister_extension_class)(
            sys::get_library(),
            class_name.string_sys(),
        );
    }
    out!("Class {class_name} unloaded");
}

/// Callbacks that are passed as function pointers to Godot upon class registration.
///
/// Re-exported to `crate::private`
#[allow(clippy::missing_safety_doc)]
pub mod callbacks {
    use super::*;
    use crate::builder::ClassBuilder;
    use crate::obj::Base;
    use crate::storage::{Storage, StorageRefCounted};

    pub unsafe extern "C" fn create<T: cap::GodotDefault>(
        _class_userdata: *mut std::ffi::c_void,
    ) -> sys::GDExtensionObjectPtr {
        create_custom(T::__godot_user_init)
    }

    #[cfg(since_api = "4.2")]
    pub unsafe extern "C" fn recreate<T: cap::GodotDefault>(
        _class_userdata: *mut std::ffi::c_void,
        object: sys::GDExtensionObjectPtr,
    ) -> sys::GDExtensionClassInstancePtr {
        create_rust_part_for_existing_godot_part(T::__godot_user_init, object)
    }

    pub(crate) fn create_custom<T, F>(make_user_instance: F) -> sys::GDExtensionObjectPtr
    where
        T: GodotClass,
        F: FnOnce(Base<T::Base>) -> T,
    {
        let base_class_name = T::Base::class_name();

        let base_ptr =
            unsafe { interface_fn!(classdb_construct_object)(base_class_name.string_sys()) };

        create_rust_part_for_existing_godot_part(make_user_instance, base_ptr);

        // std::mem::forget(base_class_name);
        base_ptr
    }

    // with GDExt, custom object consists from two parts: Godot object and Rust object, that are
    // bound to each other. this method takes the first by pointer, creates the second with
    // supplied state and binds them together. that's used for both brand-new objects creation and
    // hot reload - during hot-reload, Rust objects are disposed and then created again with a
    // updated code, so that's necessary to link them to Godot objects again.
    fn create_rust_part_for_existing_godot_part<T, F>(
        make_user_instance: F,
        base_ptr: sys::GDExtensionObjectPtr,
    ) -> sys::GDExtensionClassInstancePtr
    where
        T: GodotClass,
        F: FnOnce(Base<T::Base>) -> T,
    {
        let class_name = T::class_name();

        //out!("create callback: {}", class_name.backing);

        let base = unsafe { Base::from_sys(base_ptr) };
        let user_instance = make_user_instance(unsafe { Base::from_base(&base) });

        let instance = InstanceStorage::<T>::construct(user_instance, base);
        let instance_ptr = instance.into_raw();
        let instance_ptr = instance_ptr as sys::GDExtensionClassInstancePtr;

        let binding_data_callbacks = crate::storage::nop_instance_callbacks();
        unsafe {
            interface_fn!(object_set_instance)(base_ptr, class_name.string_sys(), instance_ptr);
            interface_fn!(object_set_instance_binding)(
                base_ptr,
                sys::get_library() as *mut std::ffi::c_void,
                instance_ptr as *mut std::ffi::c_void,
                &binding_data_callbacks,
            );
        }

        // std::mem::forget(class_name);
        instance_ptr
    }

    pub unsafe extern "C" fn free<T: GodotClass>(
        _class_user_data: *mut std::ffi::c_void,
        instance: sys::GDExtensionClassInstancePtr,
    ) {
        {
            let storage = as_storage::<T>(instance);
            storage.mark_destroyed_by_godot();
        } // Ref no longer valid once next statement is executed.

        crate::storage::destroy_storage::<T>(instance);
    }

    pub unsafe extern "C" fn get_virtual<T: cap::ImplementsGodotVirtual>(
        _class_user_data: *mut std::ffi::c_void,
        name: sys::GDExtensionConstStringNamePtr,
    ) -> sys::GDExtensionClassCallVirtual {
        // This string is not ours, so we cannot call the destructor on it.
        let borrowed_string = StringName::from_string_sys(sys::force_mut_ptr(name));
        let method_name = borrowed_string.to_string();
        std::mem::forget(borrowed_string);

        T::__virtual_call(method_name.as_str())
    }

    pub unsafe extern "C" fn default_get_virtual<T: UserClass>(
        _class_user_data: *mut std::ffi::c_void,
        name: sys::GDExtensionConstStringNamePtr,
    ) -> sys::GDExtensionClassCallVirtual {
        // This string is not ours, so we cannot call the destructor on it.
        let borrowed_string = StringName::from_string_sys(sys::force_mut_ptr(name));
        let method_name = borrowed_string.to_string();
        std::mem::forget(borrowed_string);

        T::__default_virtual_call(method_name.as_str())
    }

    pub unsafe extern "C" fn to_string<T: cap::GodotToString>(
        instance: sys::GDExtensionClassInstancePtr,
        _is_valid: *mut sys::GDExtensionBool,
        out_string: sys::GDExtensionStringPtr,
    ) {
        // Note: to_string currently always succeeds, as it is only provided for classes that have a working implementation.
        // is_valid output parameter thus not needed.

        let storage = as_storage::<T>(instance);
        let instance = storage.get();
        let string = T::__godot_to_string(&*instance);

        // Transfer ownership to Godot
        string.move_string_ptr(out_string);
    }

    #[cfg(before_api = "4.2")]
    pub unsafe extern "C" fn on_notification<T: cap::GodotNotification>(
        instance: sys::GDExtensionClassInstancePtr,
        what: i32,
    ) {
        let storage = as_storage::<T>(instance);
        let mut instance = storage.get_mut();

        T::__godot_notification(&mut *instance, what);
    }

    #[cfg(since_api = "4.2")]
    pub unsafe extern "C" fn on_notification<T: cap::GodotNotification>(
        instance: sys::GDExtensionClassInstancePtr,
        what: i32,
        _reversed: sys::GDExtensionBool,
    ) {
        let storage = as_storage::<T>(instance);
        let mut instance = storage.get_mut();

        T::__godot_notification(&mut *instance, what);
    }

    pub unsafe extern "C" fn reference<T: GodotClass>(instance: sys::GDExtensionClassInstancePtr) {
        let storage = as_storage::<T>(instance);
        storage.on_inc_ref();
    }

    pub unsafe extern "C" fn unreference<T: GodotClass>(
        instance: sys::GDExtensionClassInstancePtr,
    ) {
        let storage = as_storage::<T>(instance);
        storage.on_dec_ref();
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------------
    // Safe, higher-level methods

    /// Abstracts the `GodotDefault` away, for contexts where this trait bound is not statically available
    pub fn erased_init<T: cap::GodotDefault>(base: Box<dyn Any>) -> Box<dyn Any> {
        let concrete = base
            .downcast::<Base<<T as GodotClass>::Base>>()
            .expect("erased_init: bad type erasure");
        let extracted: Base<_> = sys::unbox(concrete);

        let instance = T::__godot_user_init(extracted);
        Box::new(instance)
    }

    pub fn register_class_by_builder<T: cap::GodotRegisterClass>(_class_builder: &mut dyn Any) {
        // TODO use actual argument, once class builder carries state
        // let class_builder = class_builder
        //     .downcast_mut::<ClassBuilder<T>>()
        //     .expect("bad type erasure");

        let mut class_builder = ClassBuilder::new();
        T::__godot_register_class(&mut class_builder);
    }

    pub fn register_user_properties<T: cap::ImplementsGodotExports>(_class_builder: &mut dyn Any) {
        T::__register_exports();
    }

    pub fn register_user_methods_constants<T: cap::ImplementsGodotApi>(
        _class_builder: &mut dyn Any,
    ) {
        // let class_builder = class_builder
        //     .downcast_mut::<ClassBuilder<T>>()
        //     .expect("bad type erasure");

        //T::register_methods(class_builder);
        T::__register_methods();
        T::__register_constants();
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Substitutes for Default impl

// Yes, bindgen can implement Default, but only for _all_ types (with single exceptions).
// For FFI types, it's better to have explicit initialization in the general case though.
fn default_registration_info(class_name: ClassName) -> ClassRegistrationInfo {
    ClassRegistrationInfo {
        class_name,
        parent_class_name: None,
        register_methods_constants_fn: None,
        register_properties_fn: None,
        user_register_fn: None,
        default_virtual_fn: None,
        user_virtual_fn: None,
        godot_params: default_creation_info(),
        init_level: InitLevel::Scene,
        is_editor_plugin: false,
    }
}

#[cfg(before_api = "4.2")]
fn default_creation_info() -> sys::GDExtensionClassCreationInfo {
    sys::GDExtensionClassCreationInfo {
        is_abstract: false as u8,
        is_virtual: false as u8,
        set_func: None,
        get_func: None,
        get_property_list_func: None,
        free_property_list_func: None,
        property_can_revert_func: None,
        property_get_revert_func: None,
        notification_func: None,
        to_string_func: None,
        reference_func: None,
        unreference_func: None,
        create_instance_func: None,
        free_instance_func: None,
        get_virtual_func: None,
        get_rid_func: None,
        class_userdata: ptr::null_mut(),
    }
}

#[cfg(since_api = "4.2")]
fn default_creation_info() -> sys::GDExtensionClassCreationInfo2 {
    sys::GDExtensionClassCreationInfo2 {
        is_abstract: false as u8,
        is_virtual: false as u8,
        set_func: None,
        get_func: None,
        get_property_list_func: None,
        free_property_list_func: None,
        property_can_revert_func: None,
        property_get_revert_func: None,
        validate_property_func: None,
        notification_func: None,
        to_string_func: None,
        reference_func: None,
        unreference_func: None,
        create_instance_func: None,
        free_instance_func: None,
        recreate_instance_func: None,
        get_virtual_func: None,
        get_virtual_call_data_func: None,
        call_virtual_with_data_func: None,
        get_rid_func: None,
        class_userdata: ptr::null_mut(),
        is_exposed: true as sys::GDExtensionBool,
    }
}
