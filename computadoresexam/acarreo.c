#include <stdio.h>

void print_binary(unsigned char n, int bits) {
    for (int i = bits - 1; i >= 0; i--) {
        printf("%d", (n >> i) & 1);
    }
}

void suma_binaria(unsigned char a, unsigned char b, int bits) {
    unsigned char carry = 0;
    unsigned char result = 0;
    printf("Suma binaria:\n");
    printf("   ");
    print_binary(a, bits);
    printf("\n+  ");
    print_binary(b, bits);
    printf("\n-----------------\n");

    for (int i = 0; i < bits; i++) {
        unsigned char bit_a = (a >> i) & 1;
        unsigned char bit_b = (b >> i) & 1;
        unsigned char sum = bit_a + bit_b + carry;
        unsigned char res_bit = sum & 1;
        carry = (sum >> 1) & 1;
        printf("Bit %d: %d + %d + carry(%d) = %d (Resultado: %d, Nuevo carry: %d)\n",
               i, bit_a, bit_b, carry, sum, res_bit, carry);
        result |= (res_bit << i);
    }
    printf("Resultado: ");
    print_binary(result, bits);
    printf("\nCarry out final: %d\n", carry);
}

void resta_binaria(unsigned char a, unsigned char b, int bits) {
    unsigned char borrow = 0;
    unsigned char result = 0;
    printf("Resta binaria:\n");
    printf("   ");
    print_binary(a, bits);
    printf("\n-  ");
    print_binary(b, bits);
    printf("\n-----------------\n");

    for (int i = 0; i < bits; i++) {
        unsigned char bit_a = (a >> i) & 1;
        unsigned char bit_b = (b >> i) & 1;
        unsigned char diff = bit_a - bit_b - borrow;
        unsigned char res_bit;
        if (diff >= 0) {
            res_bit = diff;
            borrow = 0;
        } else {
            res_bit = diff + 2;
            borrow = 1;
        }
        printf("Bit %d: %d - %d - borrow(%d) = %d (Resultado: %d, Nuevo borrow: %d)\n",
               i, bit_a, bit_b, borrow, diff, res_bit, borrow);
        result |= (res_bit << i);
    }
    printf("Resultado: ");
    print_binary(result, bits);
    printf("\nBorrow out final: %d\n", borrow);
}

int main() {
    unsigned char a = 0b1011; // 11
    unsigned char b = 0b1101; // 13
    int bits = 4;

    suma_binaria(a, b, bits);
    printf("\n");
    resta_binaria(a, b, bits);

    return 0;
}
