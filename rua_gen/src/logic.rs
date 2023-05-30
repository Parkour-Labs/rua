//! The logic of `rua`

use std::path::{Path, PathBuf};

use crate::{
    errors::{ParseError, RuaError, RuaFsError},
    models::{Rua, RuaHasAttr, RuaNamed, RuaVisible},
};
use cargo_toml_parser::{CargoToml, Package, Workspace};
use syn::File;

/// The runner for `rua`.
pub struct RuaRunner<T> {
    rua: T,
    modules: Vec<Module>,
}

/// The type of a module.
#[derive(Debug, Clone, Copy)]
pub enum RuaModuleType {
    /// Represents a crate module.
    CrateModule,
    /// Represents a file module.
    FileModule,
}

/// Represents a module.
#[derive(Debug, Clone)]
pub struct Module {
    /// Represents the name of the module.
    pub name: String,
    /// Represents the type of the module.
    pub ty: RuaModuleType,
    root_path: PathBuf,
}

impl Module {
    fn new(name: String, ty: RuaModuleType, root_path: PathBuf) -> Self {
        Self {
            name,
            ty,
            root_path,
        }
    }
}

impl<T: Rua> RuaRunner<T> {
    /// Creates a new runner.
    pub fn new(rua: T) -> Self {
        Self {
            rua,
            modules: vec![],
        }
    }

    fn read_and_parse_toml(
        path: impl AsRef<Path>,
    ) -> Result<CargoToml, RuaError> {
        let data = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            log::error!("Failed to read Cargo.toml: {}", e);
            RuaError::FsError(RuaFsError::ReadFileErr {
                path: path.as_ref().to_owned(),
                err: Box::new(e),
            })
        })?;
        data.as_str().try_into().map_err(|e| {
            log::error!("Failed to parse Cargo.toml: {}", e);
            RuaError::ParseError(ParseError {
                path: path.as_ref().to_owned(),
                err: Box::new(e),
            })
        })
    }

    fn handle_cargo_package(&mut self, package: Option<Package>) {
        if let Some(package) = package {
            if let Some(name) = package.name {
                log::info!("Package name: {}", name);
                self.modules.push(Module::new(
                    name,
                    RuaModuleType::CrateModule,
                    self.rua.entry_path().to_owned(),
                ));
            }
        }
    }

    fn handle_cargo_workspace(&mut self, workspace: Option<Workspace>) {
        if let Some(workspace) = workspace {
            for member in workspace.members {
                log::info!("Workspace member: {}", member);
                self.modules.push(Module::new(
                    member.clone(),
                    RuaModuleType::CrateModule,
                    self.rua.entry_path().join(member),
                ));
            }
        }
    }

    fn read_entry_module(&mut self) -> Result<&mut Self, RuaError> {
        let cargo_toml = Self::read_and_parse_toml(
            self.rua.entry_path().join("Cargo.toml"),
        )?;
        self.handle_cargo_package(cargo_toml.package);
        self.handle_cargo_workspace(cargo_toml.workspace);
        Ok(self)
    }

    fn get_valid_file_path(&self, module: &Module) -> Option<PathBuf> {
        let mut path = module.root_path.clone();
        // case one: path/name.rs
        // case two: path/name/mod.rs
        path.push(format!("{}.rs", module.name));
        if path.exists() {
            return Some(path);
        }
        path.pop();
        path.push(module.name.clone());
        path.push("mod.rs");
        if path.exists() {
            return Some(path);
        }
        None
    }

    fn read_file_module(
        &mut self,
        module: &Module,
    ) -> Result<&mut Self, RuaError> {
        let path = self.get_valid_file_path(module).ok_or_else(|| {
            let err = RuaFsError::FileNotFoundErr(module.name.clone().into());
            log::error!("Failed to find file module: {}", err);
            RuaError::FsError(err)
        })?;
        let data = self.read_and_parse_file(&path)?;
        self.handle_parsed_file(module, path, &data);
        Ok(self)
    }

    fn handle_item_mod(
        &mut self,
        entry_path: impl AsRef<Path>,
        item_mod: &syn::ItemMod,
    ) {
        if !item_mod.is_pub() {
            log::info!("Skipping {} because it is not public", item_mod.name());
        }
        let name = item_mod.name();
        self.modules.push(Module::new(
            name,
            RuaModuleType::FileModule,
            entry_path.as_ref().to_owned(),
        ));
    }

    fn should_include_item<K: RuaVisible + RuaHasAttr + RuaNamed>(
        &mut self,
        item: &K,
    ) -> bool {
        if !item.is_pub() {
            log::info!("Skipping {} because it is not public", item.name());
            return false;
        }
        if !self.rua.should_include(item) {
            log::info!(
                "Skipping {} because `should_include` returned false",
                item.name()
            );
            return false;
        }
        true
    }

    fn handle_item_struct(
        &mut self,
        m: &Module,
        item_struct: &syn::ItemStruct,
    ) {
        if !self.should_include_item(item_struct) {
            return;
        }
        self.rua.write_struct(m, item_struct);
    }

    fn handle_item_enum(&mut self, m: &Module, item_enum: &syn::ItemEnum) {
        if !self.should_include_item(item_enum) {
            return;
        }
        self.rua.write_enum(m, item_enum);
    }

    fn handle_item_fn(&mut self, m: &Module, item_fn: &syn::ItemFn) {
        if !self.should_include_item(item_fn) {
            return;
        }
        self.rua.write_fn(m, item_fn);
    }

    fn read_and_parse_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<File, RuaError> {
        let data = self.rua.read_file(&path).map_err(|e| {
            log::error!("Failed to read file: {}", e);
            RuaError::FsError(e)
        })?;
        let data = data.as_str();
        syn::parse_file(data).map_err(|e| {
            log::error!("Failed to parse file: {}", e);
            RuaError::ParseError(ParseError {
                path: path.as_ref().to_owned(),
                err: Box::new(e),
            })
        })
    }

    fn handle_parsed_file(
        &mut self,
        m: &Module,
        entry_path: impl AsRef<Path>,
        parsed: &File,
    ) {
        for item in &parsed.items {
            match item {
                syn::Item::Mod(item_mod) => {
                    self.handle_item_mod(&entry_path, &item_mod);
                }
                syn::Item::Struct(item_struct) => {
                    self.handle_item_struct(m, &item_struct);
                }
                syn::Item::Enum(item_enum) => {
                    self.handle_item_enum(m, &item_enum);
                }
                syn::Item::Fn(item_fn) => {
                    self.handle_item_fn(m, &item_fn);
                }
                _ => {}
            }
        }
    }

    fn read_crate_module(
        &mut self,
        module: &Module,
    ) -> Result<&mut Self, RuaError> {
        let entry_path = module.root_path.join("src");
        let file_path = entry_path.join("lib.rs");
        let parsed = self.read_and_parse_file(&file_path)?;
        self.handle_parsed_file(module, entry_path, &parsed);
        Ok(self)
    }

    fn read_module(&mut self, module: &Module) -> Result<&mut Self, RuaError> {
        match module.ty {
            RuaModuleType::CrateModule => self.read_crate_module(module),
            RuaModuleType::FileModule => self.read_file_module(module),
        }
    }

    /// Runs the runner.
    pub fn run(&mut self) -> Result<(), RuaError> {
        log::info!("Rua starting at {}", self.rua.entry_path().display());
        self.read_entry_module()?;
        while let Some(module) = self.modules.pop() {
            self.read_module(&module)?;
        }
        Ok(())
    }
}
