#include <stdio.h>
#include <stdbool.h>
#include "alu_textos.h"

// Funciones lógicas
bool and(bool a, bool b) {
    return a && b;
}

bool or(bool a, bool b) {
    return a || b;
}

bool not(bool a) {
    return !a;
}

bool xor(bool a, bool b) {
    return a != b;
}

bool nand(bool a, bool b) {
    return !(a && b);
}

// Sumador completo de 1 bit
void sumador_completo(bool a, bool b, bool cin, bool *suma, bool *cout) {
    *suma = (a != b) != cin;
    *cout = (a && b) || (cin && (a != b));
}

// Decodificador/Selector de función
void ejecutar_operacion(int opcode, bool a, bool b, bool cin) {
    printf("\n--- RESULTADO DE LA OPERACION ---\n");
    switch(opcode) {
        case 0:
            printf("Operacion: AND\n");
            printf("Descripcion: La operacion AND compara ambos bits. El resultado es 1 solo si ambos A y B son 1.\n");
            printf("Tabla de verdad AND:\n");
            printf("  A B | A AND B\n");
            printf("  0 0 |   0\n  0 1 |   0\n  1 0 |   0\n  1 1 |   1\n");
            printf("Entradas: A = %d, B = %d\n", a, b);
            printf("Salida: %d AND %d = %d\n", a, b, and(a, b));
            break;
        case 1:
            printf("Operacion: OR\n");
            printf("Descripcion: La operacion OR compara ambos bits. El resultado es 1 si al menos uno de A o B es 1.\n");
            printf("Tabla de verdad OR:\n");
            printf("  A B | A OR B\n");
            printf("  0 0 |   0\n  0 1 |   1\n  1 0 |   1\n  1 1 |   1\n");
            printf("Entradas: A = %d, B = %d\n", a, b);
            printf("Salida: %d OR %d = %d\n", a, b, or(a, b));
            break;
        case 2:
            printf("Operacion: NOT\n");
            printf("Descripcion: La operacion NOT invierte el bit de entrada. Si A es 1, el resultado es 0; si A es 0, el resultado es 1.\n");
            printf("Tabla de verdad NOT:\n");
            printf("  A | NOT A\n");
            printf("  0 |   1\n  1 |   0\n");
            printf("Entrada: A = %d (B no se usa en esta operación)\n", a);
            printf("Salida: NOT %d = %d\n", a, not(a));
            break;
        case 4:
            printf("Operacion: XOR\n");
            printf("Descripcion: La operacion XOR (O exclusivo) compara ambos bits. El resultado es 1 solo si A y B son diferentes.\n");
            printf("Tabla de verdad XOR:\n");
            printf("  A B | A XOR B\n");
            printf("  0 0 |   0\n  0 1 |   1\n  1 0 |   1\n  1 1 |   0\n");
            printf("Entradas: A = %d, B = %d\n", a, b);
            printf("Salida: %d XOR %d = %d\n", a, b, xor(a, b));
            break;
        case 5:
            printf("Operacion: NAND\n");
            printf("Descripcion: La operacion NAND es la negación de AND. El resultado es 0 solo si ambos A y B son 1; en cualquier otro caso es 1.\n");
            printf("Tabla de verdad NAND:\n");
            printf("  A B | A NAND B\n");
            printf("  0 0 |   1\n  0 1 |   1\n  1 0 |   1\n  1 1 |   0\n");
            printf("Entradas: A = %d, B = %d\n", a, b);
            printf("Salida: %d NAND %d = %d\n", a, b, nand(a, b));
            break;
        case 3: {
            bool suma, cout;
            sumador_completo(a, b, cin, &suma, &cout);
            printf("Operacion: SUMA (Sumador Completo)\n");
            printf("Descripcion: El sumador completo suma los bits A y B junto con el acarreo de entrada (CarryIn). Devuelve la suma y el acarreo de salida (CarryOut).\n");
            printf("Tabla de verdad SUMADOR COMPLETO:\n");
            printf("  A B Cin | Suma Cout\n");
            printf("  0 0  0  |  0    0\n  0 0  1  |  1    0\n  0 1  0  |  1    0\n  0 1  1  |  0    1\n  1 0  0  |  1    0\n  1 0  1  |  0    1\n  1 1  0  |  0    1\n  1 1  1  |  1    1\n");
            printf("Entradas: A = %d, B = %d, CarryIn = %d\n", a, b, cin);
            printf("Salida: Suma = %d, CarryOut = %d\n", suma, cout);
            printf("\n--- Explicación paso a paso de la suma de 1 bit ---\n");
            bool axorb = a != b;
            printf("1. Calculamos A XOR B: %d XOR %d = %d\n", a, b, axorb);
            printf("2. Calculamos (A XOR B) XOR CarryIn: %d XOR %d = %d (Suma)\n", axorb, cin, suma);
            printf("3. Calculamos CarryOut: (A AND B) OR (CarryIn AND (A XOR B)) = (%d AND %d) OR (%d AND %d) = %d OR %d = %d\n", a, b, cin, axorb, (a && b), (cin && axorb), cout);
            int resultado_decimal = (cout << 1) | suma;
            printf("\n--- Resumen de la operación ---\n");
            printf("  %d + %d + %d = %d (en decimal)\n", a, b, cin, a + b + cin);
            printf("  Resultado binario (CarryOut Suma): %d%d\n", cout, suma);
            printf("  Resultado decimal interpretando los bits: %d\n", resultado_decimal);
            printf("----------------------------------\n");
            break;
        }
        default:
            printf("Operacion no valida.\n");
    }
    printf("--------------------------------\n\n");
}

int main() {
    bool a, b, cin = 0;
    int opcode;
    char repetir = 's';
    printf(TEXTO_BIENVENIDA);
    printf("\n--------------------------------\n");
 
    printf(TEXTO_PARAMETROS_ENTRADA);
    while (repetir == 's' || repetir == 'S') {
        int valid = 0;
        do {
            printf("Ingrese el valor de A (0 o 1): ");
            int temp;
            scanf("%d", &temp);
            if (temp == 0 || temp == 1) { a = temp; valid = 1; }
            else printf("Valor invalido. A debe ser 0 o 1.\n");
        } while (!valid);
        valid = 0;
        do {
            printf("Ingrese el valor de B (0 o 1): ");
            int temp;
            scanf("%d", &temp);
            if (temp == 0 || temp == 1) { b = temp; valid = 1; }
            else printf("Valor invalido. B debe ser 0 o 1.\n");
        } while (!valid);
        valid = 0;
        do {
            printf("Seleccione la operacion a realizar:\n");
            printf("  0: AND\n");
            printf("  1: OR\n");
            printf("  2: NOT (solo usa A)\n");
            printf("  3: SUMA (Sumador Completo)\n");
            printf("  4: XOR\n");
            printf("  5: NAND\n");
            printf("Ingrese el codigo de operacion (0-5): ");
            scanf("%d", &opcode);
            if (opcode >= 0 && opcode <= 5) valid = 1;
            else printf("Valor invalido. El codigo de operacion debe estar entre 0 y 5.\n");
        } while (!valid);
        if (opcode == 3) {
            printf(TEXTO_CARRYIN);
            valid = 0;
            do {
                printf("Ingrese el valor de CarryIn (0 o 1): ");
                int temp;
                scanf("%d", &temp);
                if (temp == 0 || temp == 1) { cin = temp; valid = 1; }
                else printf("Valor invalido. CarryIn debe ser 0 o 1.\n");
            } while (!valid);
        } else {
            cin = 0;
        }
        ejecutar_operacion(opcode, a, b, cin);
        printf("¿Desea realizar otra operación? (s/n): ");
        scanf(" %c", &repetir);
    }
    printf("Gracias por usar el simulador educativo de ALU de 1 bit.\n");
    return 0;
}
