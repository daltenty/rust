# `powerpc64-ibm-aix`

**Tier: 3**

Rust for AIX operating system, currently only 64-bit PowerPC is supported.

## Target maintainers

[@daltenty](https://github.com/daltenty)
[@gilamn5tr](https://github.com/gilamn5tr)
[@amy-kwan](https://github.com/amy-kwan)

## Requirements

This target supports host tools, std and alloc. This target cannot be cross-compiled as for now, mainly because of the unavailability of system linker on other platforms.

Binaries built for this target are expected to run on Power7 or newer CPU, on AIX releases with Technology Levels (TLs) which have not yet End of Fix Support (EoFS) by IBM (https://www.ibm.com/support/pages/aix-support-lifecycle-information), which as of August 2026 are AIX 7.2 TL5 and AIX 7.3 TL4.

Binary format of this platform is [XCOFF](https://www.ibm.com/docs/en/aix/7.2?topic=formats-xcoff-object-file-format). Archive file format is ['AIX big format'](https://www.ibm.com/docs/en/aix/7.2?topic=formats-ar-file-format-big).

## Testing

This target supports running test suites natively, but it's not available to cross-compile and execute in emulator.

## Interoperability with C code

This target supports C code. C code compiled by XL, Open XL and Clang are compatible with Rust. Typical triple of AIX on 64-bit PowerPC of these compilers are also `powerpc64-ibm-aix`.
