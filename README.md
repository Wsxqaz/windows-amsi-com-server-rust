# windows-amsi-provider-rust

## prerequisites

* install rustup
* install mingw
* `rustup target add x86_64-pc-windows-gnu`

## how to build

```bash
cargo build --target=x86_64-pc-windows-gnu 2>&1
```

## how to run

```powershell
regsvr32.exe <path-to-amsi-provider.dll>
```
note: this will create the following registry keys,
```
HKLM\Software\Microsoft\AMSI\Providers\<SID in lib.rs>
HKLM\Software\Classes\CLSID\<SID in lib.rs>
HKLM\Software\Classes\CLSID\<SID in lib.rs>\InprocServer32
```
see `DllRegisterServer` in `lib.rs` for more details.

to degister the dll, run the following command,
```
regsvr32.exe -u <path-to-amsi-provider.dll>
```
see `DllUnregisterServer` in `lib.rs` for more details.

## references

* https://learn.microsoft.com/en-us/windows/win32/amsi
* https://github.com/microsoft/windows-rs/issues/1819
* https://github.com/microsoft/Windows-classic-samples/tree/main/Samples/AmsiProvider
- https://github.com/microsoft/com-rs/tree/master/examples/basic/server
