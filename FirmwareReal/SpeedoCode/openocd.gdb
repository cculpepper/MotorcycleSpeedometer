
set arch riscv:rv32
target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

load
# start
# run
# detach

c
# break _start

# start the process but immediately halt the processor

