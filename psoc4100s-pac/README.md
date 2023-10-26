# psoc4100s-pac

This is a [Peripheral Access Crate][1] for Infineon PSoC 4100S microcontrollers.


This crate has been automatically generated from the SVD file in[mtb-pdl-cat2]
[2], using [chiptool][3]. Fixes are ade to the SVD file to make the crate more
amenable to writing HALs with, such as converting sets of identical
registers/fields to arrays, merging identical registers and enums, etc.

This crate is used for the [`embassy-rp`][4] Rust Hardware Abstraction Layer
(HAL) for PSoC 4100S microcontrollers.

## Supported chips

TODO: fill out with supported chips and links to datasheets and TRMs

## License

The contents of this crate are auto-generated and licensed under the same terms
as the underlying SVD file, which is licensed by Cypress Semiconductor
Corporation under a Apache License, Version 2.0.


[1]: https://rust-embedded.github.io/book/start/registers.html
[2]: https://github.com/Infineon/mtb-pdl-cat2/blob/ea3a37451801145bbce49f7e989c683618a1cebf/devices/svd/psoc4100s.svd
[3]: https://github.com/embassy-rs/chiptool/
[4]: github.com/embassy-rs/embassy/
