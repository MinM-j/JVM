# Problem

Traditional JVMs are complex, making them difficult to understand, especially for beginners. A simplified JVM focused on core functionalities can serve as an educational tool while addressing challenges in bytecode interpretation. This project aims to create a streamlined JVM to enhance learning and research in virtual machine design. The key problems addressed are:

* Complexity of Existing JVM Implementations:
* Need for a Simplified JVM for Learning:
* Challenges in Bytecode Interpretation:

# About the Project
The project aims to develop a customized Java Virtual Machine (JVM) that executes a subset of the Java language. It will read and run class files compiled from Java source code while incorporating essential features like exception handling, stack tracing, and garbage collection. The JVM will support primitive data types, arrays, strings, control flow statements, classes, subclasses, interfaces, and both virtual and static methods. However, features like reflection, multithreading, and Just-In-Time (JIT) compilation will be excluded to focus on execution. This project enhances the understanding of JVM design and explores future research opportunities in virtual machines.

For further information, surf the [docs](./docs) section.

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



