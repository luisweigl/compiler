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
if <condition> {
    ...
} 
```

### While Statement

```c
while <condition> {
    ...
}
```
## Printing

```c
print("Hello World!");
print(a);
```

## Grammar
```
Program -> <Stmt>

<Stmt> -> <Decl> | <If> | <IfElse> | <While> | <Print>

<Decl> -> <Type> <Ident> = <Expr>;

<If> -> if (<Expr>) <Block>

<While> -> while (<Expr>) <Block>

<Block> -> { <Stmt>* }

<Expr> -> <Term> > <Expr> | <Term> < <Expr> | 

<Term>



```
