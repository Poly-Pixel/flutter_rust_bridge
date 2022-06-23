use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeImportedGenerator, IrTypeImported);

impl TypeDartGeneratorTrait for TypeImportedGenerator<'_> {
    fn api2wire_body(&self) -> Option<String> {
        None
    }

	fn imports(&self) -> String {
		format!("import '{}.dart'", self.ir.crate_id)
	}
}
