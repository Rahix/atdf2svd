atdf2svd [![crates.io page](http://meritbadge.herokuapp.com/atdf2svd)](https://crates.io/crates/atdf2svd)
========
A converter tool that converts Atmel's *atdf* files into *svd*.  The primary usecase for this is to then use the *svd* files with `svd2rust` to create safe abstractions for register access.

## Usage
```
USAGE:
    atdf2svd <atdf_path> [svd_path]
```

## Installation
Install *atdf2svd* using

```shell-session
$ cargo install -f atdf2svd
```

## Notes
### Automatic Changes
There are two "post-processors" running after the conversion ([`patch.rs`](src/atdf/patch.rs)):
- `signals_to_port_fields`: Patches the registers for all `PORTx` peripherals to contain fields for each existing pin.  Pin IDs are taken from the `<signals />` tag of the port instance.
- `remove_unsafe_cpu_regs`: Removes the `SREG`(Status Register) and `SP`(Stack Pointer) registers as they should not be safely accessible.

### Manual Changes
Unfortunately, the provided *atdf* files are often not completely correct or contain undescriptive names.  One big issue is that enumerated values are often just named `VAL_0xXX`.  I recommend patching the generated *svd* files using the patch tool written by the [stm32-rs](https://github.com/stm32-rs/stm32-rs#device-and-peripheral-yaml-format) project.

## License
`atdf2svd` is licensed under the `GPL v3` license.  See [LICENSE](LICENSE) or <https://www.gnu.org/licenses/gpl-3.0.en.html> for more info.
