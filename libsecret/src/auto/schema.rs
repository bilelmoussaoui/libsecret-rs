// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Schema(Shared<ffi::SecretSchema>);

    match fn {
        ref => |ptr| ffi::secret_schema_ref(ptr),
        unref => |ptr| ffi::secret_schema_unref(ptr),
        get_type => || ffi::secret_schema_get_type(),
    }
}

impl Schema {
    //#[doc(alias = "secret_schema_new")]
    //pub fn new(name: &str, flags: SchemaFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Schema {
    //    unsafe { TODO: call ffi:secret_schema_new() }
    //}

    //#[doc(alias = "secret_schema_newv")]
    //pub fn newv(name: &str, flags: SchemaFlags, attribute_names_and_types: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 1, id: 26 }) -> Schema {
    //    unsafe { TODO: call ffi:secret_schema_newv() }
    //}
}
