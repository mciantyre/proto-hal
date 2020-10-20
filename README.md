# proto-hal

A playground for an i.MX RT HAL. `proto-hal` provides UART, GPT, and GPIO drivers
that works across i.MX RT 1010 and 1060 chips. Unlike `imxrt-hal`, a developer
does not require a feature flag to build the crate.

```
cargo build # works in virtual workspace, which includes both proto-hal and -ral
cd proto-hal
cargo build # works in HAL crate
```

`proto-hal` depends on `proto-ral`. `proto-ral` is a hand-written fork of
`imxrt-ral`. Unlike `imxrt-ral`, which requires a chip-specific feature to
export useful APIs, `proto-ral` always exports a useful, unified API. The useful
APIs include peripheral register blocks; and the set of field values, masks, and
offsets. By designing to the common peripheral registers, a HAL implementer can
design a crate that does not require feature flags.

To see `proto-hal` in action, try the Teensy 4 example, available in `examples/teensy4`.
Though not provided, it should be trivial to demonstrate `proto-hal` on an i.MX RT 1010
system, since the feature selection is already available.

## Methods

I opened the i.MX RT reference manuals for the 1010 and 1060 chips. I studied the
register blocks for

- LPUART
- GPT
- GPIO

I searched for register, field, and bit differences. I found no differences in
the register blocks for these three peripherals across the 1010 and 1060 families.
This means that the register block and field values for those three peripherals can be reachable
from today's `proto-ral` without feature flags.

By changing the RAL foundation, we can build HAL APIs that similarly do not require
a chip selection. Build the HAL's documentation,

```
cd proto-hal
cargo doc --open
```

and observe that the UART, GPT, and GPIO drivers are available. These are the same
APIs that are available when you select a chip-specific feature. These are concrete
types, not traits which are implemented across disparate types.

## Conditional Compilation

Code that must vary, like casting pointers to `RegisterBlock`s, are
conditionally compilation in `proto-ral`. For instance, GPIO2 
on the 1010 starts at a different
address than GPIO2 on the 1060. We change the addresses at compile time.
Additional, the plurality of the peripherals may vary between chips. The 1010
only has four LPUART peripherals, while the 1060 has eight. We use conditional
compilation in `proto-ral` to account for the number of peripherals. But, this
only needs to happen when the end-user eventually chooses a chip. Features are not
required to compile a general library, and they're not required to consume
the RAL's interface.

When you enable a `proto-ral` chip feature, like `"imxrt1010"`, `proto-ral` will
export an instance API that's specific to `"imxrt1010"`. The HAL was designed
to avoid, or account for, these RAL APIs that are only available with feature selection.
You'll note that there are conditional compilation blocks
inside the HAL. But, you'll find that they do not inhibit a feature-less build, and
that they will work as expected when you select your chip.

`proto-hal` will enable `proto-ral` features on your behalf. `proto-hal` features
include

- `"imxrt1010"` for i.MX RT 1010 chips
- `"imxrt1060"` for i.MX RT 1060 chips
- `"rt"` which enables support for the `cortex-m-rt` runtime

A user who wants to build a final program for their system should select one of these features.
If the user wants runtime support, they should also enable the `"rt"` feature.

A user who wants to design a higher-level driver that works across all chips should
use `proto-hal` without enabling any features. Feature selection only happens when the user
includes `proto-hal` in their dependencies, and enables the chip-specific feature.

## Discussion

In [imxrt-rs#56](https://github.com/imxrt-rs/imxrt-rs/issues/56), we discuss a split i.MX RT Rust HAL. One
of the goals is to remove the chip-specific feature requirement. The approach described
in `proto-hal` realizes that goal, since an end user does not require a feature selection to
use `proto-hal`. `proto-hal` realizes some experimental goals noted in #56:

> - Common Instance/RegisterBlock types are generated for all chips.
> - Only the unique per chip Instance/RegisterBlock types are per chip feature flagged in.
> - This lets us build drivers against the common Instance/RegisterBlock types.
> - The drivers do not need any conditional compilation to be built.

`proto-hal` could represent the common HAL, upon which we build chip-specific HALs. A
chip-specific HAL would enable the relevant feature for `proto-hal`. The chip-specific
HAL also includes driver code that's specific to the chip.

`proto-hal` does not prohibit others from creating general libraries. In #56,
we discuss how the `imxrt-uart-log` crate would benefit from designing to the common HAL.
The UART and DMA peripherals are consistent across the considered i.MX RT chips. Therefore, the
library's implementation should not need to explicitly choose a chip. `proto-hal` shows that
users can design to the common HAL without required feature selection.
We demonstrate this in the included example.
The `delay` library designs to `proto-hal`'s GPT API, and it does not need to select a feature.
The general library is used in the `teensy4` example, which is where HAL feature selection
occurs. Likewise, an `imxrt-uart-log` developer should be able to consume the common HAL without
mandatory feature selection.

This experiment focused on simple peripherals which are consistent across i.MX RT variants. It
only considered two chip families: the 1010 and 1060 families. It's impractical to maintain
`proto-ral` by hand, espectially as we integrate additional chip families, and more complex
peripherals. An automated processes should be able to identify and handle discrepancies across register blocks
and fields with more granularity than `imxrt-ral`.
