# Lil-C

Lil-C is a subset of the C Programming Language an supports basic operations and control flow.

## Variables

The only valid Datatype is currently `int` and

```c
int a = 23;
```

## Control Flow

Currently `if` and `while` are supported.

### If Statement

```c
if (<condition>) {
    ...
} 
```

### While Statement

```c
while (<condition>) {
    ...
}
```

## Functions

The program always starts at the main function:

```c
int main() {
    ...

    return 0;
}
```

Additional functions can be definied:

```c
int foo() {
    int bar = 67;
    
    return bar;
}
```

## Printing

```c
print("Hello World!");
print(a);
```
