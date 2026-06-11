use crate::avm2::parameters::ParametersExt;
use crate::avm2::{Activation, Error, Value};
use crate::external::{Callback, ExternalInterface, Value as ExternalValue};
use crate::string::AvmString;

// Fork patch: silent no-op when ExternalInterface bridge is unavailable, instead of throwing #2067.
// Many SWFs (especially older AS3 games) call ExternalInterface.call / addCallback unguarded in
// constructors, expecting the browser bridge to exist. In Ruffle desktop standalone the bridge is
// never available, so the spec-correct throw aborts AVM2 construction and the movie never inits.
// Returning Value::Undefined matches the practical behavior of the original Flash Projector for
// content not designed for standalone, and ExternalInterface.available still returns false so
// guarded code paths take the alternate branch correctly.

pub fn call<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Value<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let name = args.get_string(activation, 0);
    if !activation.context.external_interface.available() {
        return Ok(Value::Undefined);
    }

    let external_args = args
        .iter()
        .skip(1)
        .map(|arg| ExternalValue::from_avm2(activation, arg.to_owned()))
        .collect::<Result<Vec<ExternalValue>, Error>>()?;

    let result =
        ExternalInterface::call_method(activation.context, &name.to_utf8_lossy(), &external_args);

    Ok(result.into_avm2(activation.context))
}

pub fn get_available<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Value<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(activation.context.external_interface.available().into())
}

pub fn add_callback<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Value<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let name = args.get_string(activation, 0);
    let callback = args.try_get_function(1);

    if !activation.context.external_interface.available() {
        return Ok(Value::Undefined);
    }

    if let Some(method) = callback {
        activation
            .context
            .external_interface
            .add_callback(name.to_string(), Callback::Avm2 { method });
    } else {
        activation
            .context
            .external_interface
            .remove_callback(name.to_string());
    }

    Ok(Value::Undefined)
}

pub fn get_object_id<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Value<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(id) = activation.context.external_interface.get_id() {
        Ok(AvmString::new_utf8(activation.gc(), id).into())
    } else {
        Ok(Value::Null)
    }
}
