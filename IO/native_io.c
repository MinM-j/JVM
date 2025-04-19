// native_io.c
#include <stdio.h>
#include <stdarg.h>

// Simplified JNI: no real JNIEnv, just raw pointers for now
//JNIEXPORT void JNICALL Java_ioTer_prints(const char* msg) {
void Java_ioTer_prints(const char* msg) {
    printf("%s\n", msg);
}

void Java_ioTer_printd(double number) {
    printf("%lf\n", number); 
}

void Java_ioTer_printi(int number) {
    printf("%d\n", number);
}

// Compile to .so
// gcc -shared -fPIC -o libnative_io.so native_io.c
