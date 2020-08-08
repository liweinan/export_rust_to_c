// main.c

extern void rust_print(const char*);

int main() {
    char* msg = "Hello World!";

    rust_print(msg);
    return 0;
}