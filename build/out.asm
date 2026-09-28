global _start
_start:
push rbp
mov rbp, rsp
sub rsp, 128
push 48
pop rax
mov [rbp - 8], rax
push 18
pop rax
mov [rbp - 16], rax
mov rax, [rbp - 8]
push rax
pop rax
mov [rbp - 24], rax
mov rax, [rbp - 16]
push rax
pop rax
mov [rbp - 32], rax
while_0:
mov rax, [rbp - 16]
push rax
push 0
pop rbx
pop rax
cmp rax, rbx
setne al
movzx rax, al
push rax
pop rax
cmp rax, 0
je end_while_0
mov rax, [rbp - 16]
push rax
pop rax
mov [rbp - 40], rax
mov rax, [rbp - 8]
push rax
mov rax, [rbp - 16]
push rax
pop rbx
pop rax
cqo
idiv rbx
push rdx
pop rax
mov [rbp - 16], rax
mov rax, [rbp - 40]
push rax
pop rax
mov [rbp - 8], rax
jmp while_0
end_while_0:
mov rax, [rbp - 8]
push rax
pop rax
mov [rbp - 48], rax
mov rax, [rbp - 48]
push rax
pop rdi
call print_int
mov rax, [rbp - 24]
push rax
mov rax, [rbp - 32]
push rax
pop rbx
pop rax
imul rax, rbx
push rax
mov rax, [rbp - 48]
push rax
pop rbx
pop rax
cqo
idiv rbx
push rax
pop rax
mov [rbp - 56], rax
mov rax, [rbp - 56]
push rax
pop rdi
call print_int
mov rax, [rbp - 56]
push rax
push 10
pop rbx
pop rax
cqo
idiv rbx
push rdx
push 0
pop rbx
pop rax
cmp rax, rbx
sete al
movzx rax, al
push rax
pop rax
cmp rax, 0
je endif_1
push 1
pop rdi
call print_int
endif_1:
push 0
pop rdi
call print_int
mov rsp, rbp
pop rbp
mov rax, 60
mov rdi, 0
syscall


print_int:
mov rax, rdi            ; Die Zahl holen
mov rbx, 10

push 10                 ; Newline ('\n') als Endzeichen auf den Stack

.L_div_loop:
xor rdx, rdx            ; rdx leeren für Division
div rbx                 ; rax = rax / 10, rdx = Rest (Ziffer)
add rdx, '0'            ; Ziffer in ASCII wandeln ('0' bis '9')
push rdx                ; ASCII-Zeichen direkt auf den Stack legen
test rax, rax           ; Sind noch Ziffern übrig?
jnz .L_div_loop

.L_print_loop:
; 1 Byte direkt von [rsp] per sys_write ausgeben
mov rax, 1              ; sys_write
mov rdi, 1              ; stdout
mov rsi, rsp            ; Adresse des aktuellen Zeichens
mov rdx, 1              ; Länge: 1 Byte
syscall

pop rax                 ; Zeichen vom Stack nehmen
cmp rax, 10             ; War es das Newline?
jne .L_print_loop       ; Wenn nein, nächste Ziffer drucken

ret
                
