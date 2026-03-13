# x3c
```./specs```
```4b opcode 4b address
16 instruction cap
realistic default memory 2k
simulated default memory 2k
16 address' per bank(128b)

processor internal memory 

    ##-##
opcode-address
eg. ( 01ff )


0 noop
    does nothing
1 jump
    jumps to address
2 skip
    skips if address = 0xff
3 take
    takes address to buffer
4 place
    places buffer to address
5 bank up
    goes up a memory bank
6 bank down
    goes down a memory bank
7 and
    bitwise and between buffer and address stores result in address
8 or
    bitwise or ..
9 xor
    bitwise xor ..
a nor
    bitwise nor ..
b save
    save selected memory bank to storage current address  
c load
    load selected address to current memory bank
d add
    bitwise add ..
e subtract
    bitwise subtract ..
f display
    print address to display```
