target extended-remote :3333
set print asm-demangle on
set backtrace limit 32

break DefaultHandler
break HardFault
break rust_begin_unwind
break main

monitor arm semihosting enable

monitor tpiu config internal itm.tx uart off 64000000
monitor itm port 0 on

load

stepi
