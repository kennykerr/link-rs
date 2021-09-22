cl /LD export.cpp
dumpbin /exports export.lib
rustc import.rs --target i686-pc-windows-msvc
import.exe