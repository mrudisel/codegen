use crate::enum_gen::Enum;
use crate::function::Function;
use crate::impl_gen::Impl;
use crate::module::Module;
use crate::struct_gen::Struct;
use crate::trait_gen::Trait;

#[derive(Debug, Clone)]
pub enum Item {
    Module(Module),
    Struct(Struct),
    Function(Function),
    Trait(Trait),
    Enum(Enum),
    Impl(Impl),
    Raw(String),
}
