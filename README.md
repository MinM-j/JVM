### steps to run:
- first compile any `.java` file in test directory
- then run command `cargo run *.class` in the project directory
- then check the output with `javap -v *.class`

### Todos
While writing implementation for `Instruction`, change the operand type for instructions 
using tuple of u8s  necessary depending on the semantic meaning of underlying tuple.
And change the parsing part correspondingly.
- For example: `invokespecial(u8,u8)` should be changed to `invokespecial(u16)` 
    since (u8,u8) represent index of contant pool which is single value.


### Accomplished
- can parse till `fields_count` of class file with some exceptions



