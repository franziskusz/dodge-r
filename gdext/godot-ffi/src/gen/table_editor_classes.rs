type FetchFn = < crate::GDExtensionInterfaceClassdbGetMethodBind as crate::Inner > ::FnPtr;
pub struct ClassEditorMethodTable {
    function_pointers: Vec < crate::ClassMethodBind >,
}
impl ClassEditorMethodTable {
    pub const CLASS_COUNT: usize = 34usize;
    pub const METHOD_COUNT: usize = 318usize;
    pub fn load(interface: &crate::GDExtensionInterface, string_names: &mut crate::StringCache,) -> Self {
        let fetch_fptr = interface.classdb_get_method_bind.expect("classdb_get_method_bind absent");
        let mut function_pointers = Vec::with_capacity(318usize);
        load_EditorCommandPalette_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorDebuggerPlugin_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorDebuggerSession_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorExportPlatform_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorExportPlugin_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorFeatureProfile_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorFileDialog_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorFileSystem_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorFileSystemDirectory_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorImportPlugin_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorInspector_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorInspectorPlugin_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorInterface_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorNode3DGizmo_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorNode3DGizmoPlugin_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorPaths_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorPlugin_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorProperty_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorResourcePicker_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorResourcePreview_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorScenePostImport_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorScenePostImportPlugin_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorScript_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorScriptPicker_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorSelection_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorSettings_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorSpinSlider_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorUndoRedoManager_methods(&mut function_pointers, string_names, fetch_fptr);
        load_EditorVCSInterface_methods(&mut function_pointers, string_names, fetch_fptr);
        load_FileSystemDock_methods(&mut function_pointers, string_names, fetch_fptr);
        load_ResourceImporterOggVorbis_methods(&mut function_pointers, string_names, fetch_fptr);
        load_ScriptCreateDialog_methods(&mut function_pointers, string_names, fetch_fptr);
        load_ScriptEditor_methods(&mut function_pointers, string_names, fetch_fptr);
        load_ScriptEditorBase_methods(&mut function_pointers, string_names, fetch_fptr);
        Self {
            function_pointers
        }
    }
    #[inline(always)]
    pub fn fptr_by_index(&self, index: usize) -> crate::ClassMethodBind {
        unsafe {
            * self.function_pointers.get_unchecked(index)
        }
    }
}
fn load_EditorCommandPalette_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorCommandPalette = string_names.fetch("EditorCommandPalette");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorCommandPalette), "EditorCommandPalette", "add_command", 864043298i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorCommandPalette), "EditorCommandPalette", "remove_command", 83702148i64),);
    
}
fn load_EditorDebuggerPlugin_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorDebuggerPlugin = string_names.fetch("EditorDebuggerPlugin");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorDebuggerPlugin), "EditorDebuggerPlugin", "get_session", 3061968499i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorDebuggerPlugin), "EditorDebuggerPlugin", "get_sessions", 2915620761i64),);
    
}
fn load_EditorDebuggerSession_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorDebuggerSession = string_names.fetch("EditorDebuggerSession");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorDebuggerSession), "EditorDebuggerSession", "send_message", 85656714i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorDebuggerSession), "EditorDebuggerSession", "toggle_profiler", 1198443697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorDebuggerSession), "EditorDebuggerSession", "is_breaked", 2240911060i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorDebuggerSession), "EditorDebuggerSession", "is_debuggable", 2240911060i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorDebuggerSession), "EditorDebuggerSession", "is_active", 2240911060i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorDebuggerSession), "EditorDebuggerSession", "add_session_tab", 1496901182i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorDebuggerSession), "EditorDebuggerSession", "remove_session_tab", 1496901182i64),);
    
}
fn load_EditorExportPlatform_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorExportPlatform = string_names.fetch("EditorExportPlatform");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlatform), "EditorExportPlatform", "get_os_name", 201670096i64),);
    
}
fn load_EditorExportPlugin_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorExportPlugin = string_names.fetch("EditorExportPlugin");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "add_shared_object", 3098291045i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "add_ios_project_static_lib", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "add_file", 527928637i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "add_ios_framework", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "add_ios_embedded_framework", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "add_ios_plist_content", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "add_ios_linker_flags", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "add_ios_bundle_file", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "add_ios_cpp_code", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "add_macos_plugin_file", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "skip", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorExportPlugin), "EditorExportPlugin", "get_option", 2760726917i64),);
    
}
fn load_EditorFeatureProfile_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorFeatureProfile = string_names.fetch("EditorFeatureProfile");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFeatureProfile), "EditorFeatureProfile", "set_disable_class", 2524380260i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFeatureProfile), "EditorFeatureProfile", "is_class_disabled", 2619796661i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFeatureProfile), "EditorFeatureProfile", "set_disable_class_editor", 2524380260i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFeatureProfile), "EditorFeatureProfile", "is_class_editor_disabled", 2619796661i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFeatureProfile), "EditorFeatureProfile", "set_disable_class_property", 865197084i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFeatureProfile), "EditorFeatureProfile", "is_class_property_disabled", 471820014i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFeatureProfile), "EditorFeatureProfile", "set_disable_feature", 1884871044i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFeatureProfile), "EditorFeatureProfile", "is_feature_disabled", 2974403161i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFeatureProfile), "EditorFeatureProfile", "get_feature_name", 3401335809i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFeatureProfile), "EditorFeatureProfile", "save_to_file", 166001499i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFeatureProfile), "EditorFeatureProfile", "load_from_file", 166001499i64),);
    
}
fn load_EditorFileDialog_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorFileDialog = string_names.fetch("EditorFileDialog");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "clear_filters", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "add_filter", 3388804757i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "set_filters", 4015028928i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "get_filters", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "get_current_dir", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "get_current_file", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "get_current_path", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "set_current_dir", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "set_current_file", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "set_current_path", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "set_file_mode", 274150415i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "get_file_mode", 2681044145i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "get_vbox", 915758477i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "get_line_edit", 4071694264i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "set_access", 3882893764i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "get_access", 778734016i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "set_show_hidden_files", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "is_showing_hidden_files", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "set_display_mode", 3049004050i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "get_display_mode", 3517174669i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "set_disable_overwrite_warning", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "is_overwrite_warning_disabled", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "add_side_menu", 402368861i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileDialog), "EditorFileDialog", "invalidate", 3218959716i64),);
    
}
fn load_EditorFileSystem_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorFileSystem = string_names.fetch("EditorFileSystem");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystem), "EditorFileSystem", "get_filesystem", 842323275i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystem), "EditorFileSystem", "is_scanning", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystem), "EditorFileSystem", "get_scanning_progress", 1740695150i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystem), "EditorFileSystem", "scan", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystem), "EditorFileSystem", "scan_sources", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystem), "EditorFileSystem", "update_file", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystem), "EditorFileSystem", "get_filesystem_path", 3188521125i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystem), "EditorFileSystem", "get_file_type", 3135753539i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystem), "EditorFileSystem", "reimport_files", 4015028928i64),);
    
}
fn load_EditorFileSystemDirectory_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorFileSystemDirectory = string_names.fetch("EditorFileSystemDirectory");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_subdir_count", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_subdir", 2330964164i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_file_count", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_file", 844755477i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_file_path", 844755477i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_file_type", 659327637i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_file_script_class_name", 844755477i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_file_script_class_extends", 844755477i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_file_import_is_valid", 1116898809i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_name", 2841200299i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_path", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "get_parent", 842323275i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "find_file_index", 1321353865i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorFileSystemDirectory), "EditorFileSystemDirectory", "find_dir_index", 1321353865i64),);
    
}
fn load_EditorImportPlugin_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorImportPlugin = string_names.fetch("EditorImportPlugin");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorImportPlugin), "EditorImportPlugin", "append_import_external_resource", 320493106i64),);
    
}
fn load_EditorInspector_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorInspector = string_names.fetch("EditorInspector");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInspector), "EditorInspector", "get_selected_path", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInspector), "EditorInspector", "get_edited_object", 2050059866i64),);
    
}
fn load_EditorInspectorPlugin_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorInspectorPlugin = string_names.fetch("EditorInspectorPlugin");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInspectorPlugin), "EditorInspectorPlugin", "add_custom_control", 1496901182i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInspectorPlugin), "EditorInspectorPlugin", "add_property_editor", 3406284123i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInspectorPlugin), "EditorInspectorPlugin", "add_property_editor_for_multiple_properties", 788598683i64),);
    
}
fn load_EditorInterface_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorInterface = string_names.fetch("EditorInterface");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "restart_editor", 3216645846i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_command_palette", 2471163807i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_resource_filesystem", 780151678i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_editor_paths", 1595760068i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_resource_previewer", 943486957i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_selection", 2690272531i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_editor_settings", 4086932459i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "make_mesh_previews", 878078554i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "set_plugin_enabled", 2678287736i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "is_plugin_enabled", 3927539163i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_editor_theme", 3846893731i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_base_control", 2783021301i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_editor_main_screen", 1706218421i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_script_editor", 90868003i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_editor_viewport_2d", 3750751911i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_editor_viewport_3d", 1970834490i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "set_main_screen_editor", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "set_distraction_free_mode", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "is_distraction_free_mode_enabled", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_editor_scale", 1740695150i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "popup_dialog", 2015770942i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "popup_dialog_centered", 346557367i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "popup_dialog_centered_ratio", 2093669136i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "popup_dialog_centered_clamped", 3763385571i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_current_feature_profile", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "set_current_feature_profile", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_file_system_dock", 3751012327i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "select_file", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_selected_paths", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_current_path", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_current_directory", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_inspector", 3517113938i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "inspect_object", 127962172i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "edit_resource", 968641751i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "edit_node", 1078189570i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "edit_script", 219829402i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "open_scene_from_path", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "reload_scene_from_path", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_open_scenes", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_edited_scene_root", 3160264692i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "save_scene", 166280745i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "save_scene_as", 3647332257i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "save_all_scenes", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "mark_scene_as_unsaved", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "play_main_scene", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "play_current_scene", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "play_custom_scene", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "stop_playing_scene", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "is_playing_scene", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "get_playing_scene", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "set_movie_maker_enabled", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorInterface), "EditorInterface", "is_movie_maker_enabled", 36873697i64),);
    
}
fn load_EditorNode3DGizmo_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorNode3DGizmo = string_names.fetch("EditorNode3DGizmo");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "add_lines", 2910971437i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "add_mesh", 1579955111i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "add_collision_segments", 334873810i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "add_collision_triangles", 54901064i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "add_unscaled_billboard", 520007164i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "add_handles", 2254560097i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "set_node_3d", 1078189570i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "get_node_3d", 151077316i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "get_plugin", 4250544552i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "clear", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "set_hidden", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "is_subgizmo_selected", 1116898809i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmo), "EditorNode3DGizmo", "get_subgizmo_selection", 1930428628i64),);
    
}
fn load_EditorNode3DGizmoPlugin_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorNode3DGizmoPlugin = string_names.fetch("EditorNode3DGizmoPlugin");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmoPlugin), "EditorNode3DGizmoPlugin", "create_material", 3486012546i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmoPlugin), "EditorNode3DGizmoPlugin", "create_icon_material", 3804976916i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmoPlugin), "EditorNode3DGizmoPlugin", "create_handle_material", 2486475223i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmoPlugin), "EditorNode3DGizmoPlugin", "add_material", 1374068695i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorNode3DGizmoPlugin), "EditorNode3DGizmoPlugin", "get_material", 974464017i64),);
    
}
fn load_EditorPaths_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorPaths = string_names.fetch("EditorPaths");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPaths), "EditorPaths", "get_data_dir", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPaths), "EditorPaths", "get_config_dir", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPaths), "EditorPaths", "get_cache_dir", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPaths), "EditorPaths", "is_self_contained", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPaths), "EditorPaths", "get_self_contained_file", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPaths), "EditorPaths", "get_project_settings_dir", 201670096i64),);
    
}
fn load_EditorPlugin_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorPlugin = string_names.fetch("EditorPlugin");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_control_to_container", 3092750152i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_control_to_bottom_panel", 3526039376i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_control_to_dock", 3354871258i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_control_from_docks", 1496901182i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_control_from_bottom_panel", 1496901182i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_control_from_container", 3092750152i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_tool_menu_item", 2137474292i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_tool_submenu_item", 1019428915i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_tool_menu_item", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "get_export_as_menu", 1775878644i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_custom_type", 1986814599i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_custom_type", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_autoload_singleton", 3186203200i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_autoload_singleton", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "update_overlays", 3905245786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "make_bottom_panel_item_visible", 1496901182i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "hide_bottom_panel", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "get_undo_redo", 773492341i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_undo_redo_inspector_hook_callback", 1611583062i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_undo_redo_inspector_hook_callback", 1611583062i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "queue_save_layout", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_translation_parser_plugin", 3116463128i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_translation_parser_plugin", 3116463128i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_import_plugin", 3113975762i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_import_plugin", 2312482773i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_scene_format_importer_plugin", 2764104752i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_scene_format_importer_plugin", 2637776123i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_scene_post_import_plugin", 3492436322i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_scene_post_import_plugin", 3045178206i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_export_plugin", 4095952207i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_export_plugin", 4095952207i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_node_3d_gizmo_plugin", 1541015022i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_node_3d_gizmo_plugin", 1541015022i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_inspector_plugin", 546395733i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_inspector_plugin", 546395733i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_resource_conversion_plugin", 2124849111i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_resource_conversion_plugin", 2124849111i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "set_input_event_forwarding_always_enabled", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "set_force_draw_over_forwarding_enabled", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "get_editor_interface", 4223731786i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "get_script_create_dialog", 3121871482i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "add_debugger_plugin", 3749880309i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "remove_debugger_plugin", 3749880309i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorPlugin), "EditorPlugin", "get_plugin_version", 201670096i64),);
    
}
fn load_EditorProperty_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorProperty = string_names.fetch("EditorProperty");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "set_label", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "get_label", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "set_read_only", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "is_read_only", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "set_checkable", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "is_checkable", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "set_checked", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "is_checked", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "set_draw_warning", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "is_draw_warning", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "set_keying", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "is_keying", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "set_deletable", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "is_deletable", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "get_edited_property", 2002593661i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "get_edited_object", 2050059866i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "update_property", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "add_focusable", 1496901182i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "set_bottom_editor", 1496901182i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorProperty), "EditorProperty", "emit_changed", 3069422438i64),);
    
}
fn load_EditorResourcePicker_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorResourcePicker = string_names.fetch("EditorResourcePicker");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePicker), "EditorResourcePicker", "set_base_type", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePicker), "EditorResourcePicker", "get_base_type", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePicker), "EditorResourcePicker", "get_allowed_types", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePicker), "EditorResourcePicker", "set_edited_resource", 968641751i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePicker), "EditorResourcePicker", "get_edited_resource", 2674603643i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePicker), "EditorResourcePicker", "set_toggle_mode", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePicker), "EditorResourcePicker", "is_toggle_mode", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePicker), "EditorResourcePicker", "set_toggle_pressed", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePicker), "EditorResourcePicker", "set_editable", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePicker), "EditorResourcePicker", "is_editable", 36873697i64),);
    
}
fn load_EditorResourcePreview_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorResourcePreview = string_names.fetch("EditorResourcePreview");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePreview), "EditorResourcePreview", "queue_resource_preview", 233177534i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePreview), "EditorResourcePreview", "queue_edited_resource_preview", 1608376650i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePreview), "EditorResourcePreview", "add_preview_generator", 332288124i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePreview), "EditorResourcePreview", "remove_preview_generator", 332288124i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorResourcePreview), "EditorResourcePreview", "check_for_invalidation", 83702148i64),);
    
}
fn load_EditorScenePostImport_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorScenePostImport = string_names.fetch("EditorScenePostImport");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorScenePostImport), "EditorScenePostImport", "get_source_file", 201670096i64),);
    
}
fn load_EditorScenePostImportPlugin_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorScenePostImportPlugin = string_names.fetch("EditorScenePostImportPlugin");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorScenePostImportPlugin), "EditorScenePostImportPlugin", "get_option_value", 2760726917i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorScenePostImportPlugin), "EditorScenePostImportPlugin", "add_import_option", 402577236i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorScenePostImportPlugin), "EditorScenePostImportPlugin", "add_import_option_advanced", 3674075649i64),);
    
}
fn load_EditorScript_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorScript = string_names.fetch("EditorScript");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorScript), "EditorScript", "add_root_node", 1078189570i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorScript), "EditorScript", "get_scene", 3160264692i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorScript), "EditorScript", "get_editor_interface", 1976662476i64),);
    
}
fn load_EditorScriptPicker_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorScriptPicker = string_names.fetch("EditorScriptPicker");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorScriptPicker), "EditorScriptPicker", "set_script_owner", 1078189570i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorScriptPicker), "EditorScriptPicker", "get_script_owner", 3160264692i64),);
    
}
fn load_EditorSelection_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorSelection = string_names.fetch("EditorSelection");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSelection), "EditorSelection", "clear", 3218959716i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSelection), "EditorSelection", "add_node", 1078189570i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSelection), "EditorSelection", "remove_node", 1078189570i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSelection), "EditorSelection", "get_selected_nodes", 2915620761i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSelection), "EditorSelection", "get_transformable_selected_nodes", 2915620761i64),);
    
}
fn load_EditorSettings_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorSettings = string_names.fetch("EditorSettings");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "has_setting", 3927539163i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "set_setting", 402577236i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "get_setting", 1868160156i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "erase", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "set_initial_value", 1529169264i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "add_property_info", 4155329257i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "set_project_metadata", 2504492430i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "get_project_metadata", 89809366i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "set_favorites", 4015028928i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "get_favorites", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "set_recent_dirs", 4015028928i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "get_recent_dirs", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "set_builtin_action_override", 1209351045i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "check_changed_settings_in_group", 3927539163i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "get_changed_settings", 1139954409i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSettings), "EditorSettings", "mark_setting_changed", 83702148i64),);
    
}
fn load_EditorSpinSlider_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorSpinSlider = string_names.fetch("EditorSpinSlider");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSpinSlider), "EditorSpinSlider", "set_label", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSpinSlider), "EditorSpinSlider", "get_label", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSpinSlider), "EditorSpinSlider", "set_suffix", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSpinSlider), "EditorSpinSlider", "get_suffix", 201670096i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSpinSlider), "EditorSpinSlider", "set_read_only", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSpinSlider), "EditorSpinSlider", "is_read_only", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSpinSlider), "EditorSpinSlider", "set_flat", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSpinSlider), "EditorSpinSlider", "is_flat", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSpinSlider), "EditorSpinSlider", "set_hide_slider", 2586408642i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorSpinSlider), "EditorSpinSlider", "is_hiding_slider", 36873697i64),);
    
}
fn load_EditorUndoRedoManager_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorUndoRedoManager = string_names.fetch("EditorUndoRedoManager");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorUndoRedoManager), "EditorUndoRedoManager", "create_action", 2107025470i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorUndoRedoManager), "EditorUndoRedoManager", "commit_action", 3216645846i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorUndoRedoManager), "EditorUndoRedoManager", "is_committing_action", 36873697i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorUndoRedoManager), "EditorUndoRedoManager", "add_do_method", 1517810467i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorUndoRedoManager), "EditorUndoRedoManager", "add_undo_method", 1517810467i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorUndoRedoManager), "EditorUndoRedoManager", "add_do_property", 1017172818i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorUndoRedoManager), "EditorUndoRedoManager", "add_undo_property", 1017172818i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorUndoRedoManager), "EditorUndoRedoManager", "add_do_reference", 3975164845i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorUndoRedoManager), "EditorUndoRedoManager", "add_undo_reference", 3975164845i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorUndoRedoManager), "EditorUndoRedoManager", "get_object_history_id", 1107568780i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorUndoRedoManager), "EditorUndoRedoManager", "get_history_undo_redo", 2417974513i64),);
    
}
fn load_EditorVCSInterface_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_EditorVCSInterface = string_names.fetch("EditorVCSInterface");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorVCSInterface), "EditorVCSInterface", "create_diff_line", 2901184053i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorVCSInterface), "EditorVCSInterface", "create_diff_hunk", 3784842090i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorVCSInterface), "EditorVCSInterface", "create_diff_file", 2723227684i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorVCSInterface), "EditorVCSInterface", "create_commit", 1075983584i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorVCSInterface), "EditorVCSInterface", "create_status_file", 1083471673i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorVCSInterface), "EditorVCSInterface", "add_diff_hunks_into_diff_file", 4015243225i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorVCSInterface), "EditorVCSInterface", "add_line_diffs_into_diff_hunk", 4015243225i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_EditorVCSInterface), "EditorVCSInterface", "popup_error", 83702148i64),);
    
}
fn load_FileSystemDock_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_FileSystemDock = string_names.fetch("FileSystemDock");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_FileSystemDock), "FileSystemDock", "navigate_to_path", 83702148i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_FileSystemDock), "FileSystemDock", "add_resource_tooltip_plugin", 2258356838i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_FileSystemDock), "FileSystemDock", "remove_resource_tooltip_plugin", 2258356838i64),);
    
}
fn load_ResourceImporterOggVorbis_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_ResourceImporterOggVorbis = string_names.fetch("ResourceImporterOggVorbis");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ResourceImporterOggVorbis), "ResourceImporterOggVorbis", "load_from_buffer", 354904730i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ResourceImporterOggVorbis), "ResourceImporterOggVorbis", "load_from_file", 797568536i64),);
    
}
fn load_ScriptCreateDialog_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_ScriptCreateDialog = string_names.fetch("ScriptCreateDialog");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ScriptCreateDialog), "ScriptCreateDialog", "config", 869314288i64),);
    
}
fn load_ScriptEditor_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_ScriptEditor = string_names.fetch("ScriptEditor");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ScriptEditor), "ScriptEditor", "get_current_editor", 1906266726i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ScriptEditor), "ScriptEditor", "get_open_script_editors", 3995934104i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ScriptEditor), "ScriptEditor", "register_syntax_highlighter", 1092774468i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ScriptEditor), "ScriptEditor", "unregister_syntax_highlighter", 1092774468i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ScriptEditor), "ScriptEditor", "goto_line", 1286410249i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ScriptEditor), "ScriptEditor", "get_current_script", 2146468882i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ScriptEditor), "ScriptEditor", "get_open_scripts", 3995934104i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ScriptEditor), "ScriptEditor", "open_script_create_dialog", 3186203200i64),);
    
}
fn load_ScriptEditorBase_methods(function_pointers: &mut Vec < crate::ClassMethodBind >, string_names: &mut crate::StringCache, fetch_fptr: FetchFn,) {
    let sname_ScriptEditorBase = string_names.fetch("ScriptEditorBase");
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ScriptEditorBase), "ScriptEditorBase", "get_base_editor", 2783021301i64),);
    function_pointers.push(crate::load_class_method(fetch_fptr, string_names, Some(sname_ScriptEditorBase), "ScriptEditorBase", "add_syntax_highlighter", 1092774468i64),);
    
}