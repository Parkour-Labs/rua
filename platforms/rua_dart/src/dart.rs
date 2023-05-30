use rua_gen::{
    models::{Rua, RuaMod},
    utils::RuaCaseConverter,
};

use crate::config::RuaConfig;

#[derive(Debug)]
pub struct RuaDart {
    config: RuaConfig,
    type_defs: Vec<String>,
    fn_defs: Vec<String>,
    class_defs: Vec<String>,
    enum_defs: Vec<String>,
}

impl Rua for RuaDart {
    fn entry_path(&self) -> std::path::PathBuf {
        self.config.get_native_entry().into()
    }

    fn write_fn<T: rua_gen::models::RuaFn>(
        &mut self,
        m: &rua_gen::logic::Module,
        f: &T,
    ) {
        let native_type_name =
            format!("{}_{}", m.name.to_snake_case(), f.name().to_snake_case());
        let dart_type_name = native_type_name.to_pascal_case();
        let dart_name = native_type_name.to_camel_case();
    }

    fn write_struct<T: rua_gen::models::RuaStruct>(
        &mut self,
        m: &rua_gen::logic::Module,
        s: &T,
    ) {
        todo!()
    }

    fn write_enum<T: rua_gen::models::RuaEnum>(
        &mut self,
        m: &rua_gen::logic::Module,
        e: &T,
    ) {
        todo!()
    }
}
