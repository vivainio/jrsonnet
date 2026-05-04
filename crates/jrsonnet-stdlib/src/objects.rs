use jrsonnet_evaluator::{
	IStr, ObjValue, ObjValueBuilder,
	function::builtin,
	gc::WithCapacityExt,
	rustc_hash::FxHashSet,
	val::{ArrValue, Val},
};

#[builtin]
pub fn builtin_object_fields_ex(
	obj: ObjValue,
	hidden: bool,
) -> Vec<Val> {
	let out = obj.fields_ex(hidden);
	out.into_iter().map(Val::string).collect::<Vec<_>>()
}

#[builtin]
pub fn builtin_object_fields(
	o: ObjValue,
) -> Vec<Val> {
	builtin_object_fields_ex(o, false)
}

#[builtin]
pub fn builtin_object_fields_all(
	o: ObjValue,
) -> Vec<Val> {
	builtin_object_fields_ex(o, true)
}

pub fn builtin_object_values_ex(
	o: ObjValue,
	include_hidden: bool,
) -> ArrValue {
	o.values_ex(include_hidden)
}
#[builtin]
pub fn builtin_object_values(
	o: ObjValue,
) -> ArrValue {
	builtin_object_values_ex(o, false)
}
#[builtin]
pub fn builtin_object_values_all(
	o: ObjValue,
) -> ArrValue {
	builtin_object_values_ex(o, true)
}

pub fn builtin_object_keys_values_ex(
	o: ObjValue,
	include_hidden: bool,
) -> ArrValue {
	o.key_values_ex(include_hidden)
}
#[builtin]
pub fn builtin_object_keys_values(
	o: ObjValue,
) -> ArrValue {
	builtin_object_keys_values_ex(o, false)
}
#[builtin]
pub fn builtin_object_keys_values_all(
	o: ObjValue,
) -> ArrValue {
	builtin_object_keys_values_ex(o, true)
}

#[builtin]
pub fn builtin_object_has_ex(obj: ObjValue, fname: IStr, hidden: bool) -> bool {
	obj.has_field_ex(fname, hidden)
}

#[builtin]
pub fn builtin_object_has(o: ObjValue, f: IStr) -> bool {
	o.has_field(f)
}

#[builtin]
pub fn builtin_object_has_all(o: ObjValue, f: IStr) -> bool {
	o.has_field_include_hidden(f)
}

#[builtin]
pub fn builtin_object_remove_key(obj: ObjValue, key: IStr) -> ObjValue {
	let mut omit = FxHashSet::with_capacity(1);
	omit.insert(key);

	let mut out = ObjValueBuilder::new();
	out.with_super(obj).with_fields_omitted(omit);
	out.build()
}
