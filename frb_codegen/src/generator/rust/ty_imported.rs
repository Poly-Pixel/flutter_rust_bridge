use crate::generator::rust::ty::*;
use crate::generator::rust::ExternFuncCollector;
use crate::ir::*;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeImportedGenerator, IrTypeImported);

// TODO: Fix compiler errors on rust side, look to dart side for guidance

impl TypeRustGeneratorTrait for TypeImportedGenerator<'_> {
    fn wire2api_body(&self) -> Option<String> {
		None
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
		None
    }

    fn static_checks(&self) -> Option<String> {
		None
    }

    fn wrapper_struct(&self) -> Option<String> {
        None
    }

    fn wrap_obj(&self, obj: String) -> String {
		obj
    }

    fn impl_intodart(&self) -> String {
        "".to_string()
    }

    fn new_with_nullptr(&self, _collector: &mut ExternFuncCollector) -> String {
        "".to_string()
    }

    fn imports(&self) -> Option<String> {
		None
    }
}
