use crate::pyobject::{PyContext, PyFuncArgs, PyResult, TypeProtocol};
use crate::vm::VirtualMachine;

pub fn init(context: &PyContext) {
    let none_type = &context.none.typ();
    context.set_attr(&none_type, "__new__", context.new_rustfunc(none_new));
    context.set_attr(&none_type, "__repr__", context.new_rustfunc(none_repr));
    context.set_attr(&none_type, "__bool__", context.new_rustfunc(none_bool));
}

fn none_new(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [(_zelf, Some(vm.ctx.type_type.clone()))]
    );
    Ok(vm.get_none())
}

fn none_repr(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(_zelf, Some(vm.ctx.none().typ()))]);
    Ok(vm.ctx.new_str("None".to_string()))
}

fn none_bool(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(_zelf, Some(vm.ctx.none().typ()))]);
    Ok(vm.ctx.new_bool(false))
}
