# MyAnalogProcessor
Simulating an analog processor 
## Noise generated 
Analog computers generate noise over time, these are the assumtions made in this simulation
0.4% per transferre 
0.5% per 100ms (this simulation assumes the processor is 2.5MHz)
with digitalisation needed every 2% of noise to avoid losing the signal
## What am I trying to simulate 
I'm trying to simulate the 'Gepanapa' (https://personalpages.manchester.ac.uk/staff/p.dudek/projects/gepanapa/)
## Running the code your self
The excutable ./analog_processor/src/main will run the simualtor, it will prop you to enter a file name. 
- Keep it blank if you want to run the three test cases
- Type "ExampleCode" if you want to run ./code/ExampleCode.txt
- Write your own code and add it to ./code as a txt 
## Instructions
| Instruction | Input | Description
| -------- | -------- | --------------- 
| ADD | store, r1, r2 | Add r1 and r2 store result 
| SUB | store, r1, r2 | Subsitute r1 and r2 store result 
| MUL | store, r1, r2 | Multiply r1 and r2 store result 
| DIV | store, r1, r2 | Divide r1 and r2 store result 
| ADDI | store, r1, value | Add r1 and value store result 
| COMP | store, r1, r2 | Stores -1 if <, 0 if ==, 1 if >
| LDC | store, r1 | Load r1 into store
| LD | storeValueInRegister, index | Load register via index
| BEQ | destination, r1, r2 | if r1 == r2 move PC to destination
| BNE | destination, r1, r2 | if r1 != r2 move PC to destination
| STR | index, storeValueInRegister | Store via index
| JUMP | destination | Move PC to destination
| NOP | Ends Process | Ends process 
