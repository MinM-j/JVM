// native_io.c
#include <stdio.h>
#include <stdarg.h>

// Simplified JNI: no real JNIEnv, just raw pointers for now
//JNIEXPORT void JNICALL Java_ioTer_prints(const char* msg) {
void Java_ioTer_prints(const char* msg) {
    printf("%s\n", msg);
}

//JNIEXPORT void Java_ioTer_printf(const char* format, ...) {
void Java_ioTer_printf(const char* format, ...) {
    va_list args;
    va_start(args, format);
    vprintf(format, args); // Use vprintf for variadic printing
    va_end(args);
    putchar('\n'); // Add newline like Rust's println!
}

int Java_ioTer_add(int a, int b) {
    return a + b;
}

void Java_ioTer_printn(double number) {
    printf("%lf\n", number); // Prints as double, handles all numeric types
}

void Java_ioTer_printi(int number) {
    printf("%d\n", number);
}

// Compile to .so
// gcc -shared -fPIC -o libnative_io.so native_io.c
