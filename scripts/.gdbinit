set disassembly-flavor intel
define hook-stop
x/i $rip
end
target remote localhost:1234
b main

