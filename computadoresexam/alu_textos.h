#ifndef ALU_TEXTOS_H
#define ALU_TEXTOS_H

#define TEXTO_BIENVENIDA "" \
"" \
"" \
"" \
"\n----------------------------------\n\n" \
"  - AND: Operación lógica que da 1 solo si ambos bits de entrada son 1.\n" \
"  - OR: Operación lógica que da 1 si al menos uno de los bits de entrada es 1.\n" \
"  - NOT: Operación lógica que invierte el valor del bit de entrada.\n" \
"  - XOR: Operación lógica que da 1 solo si los bits de entrada son diferentes.\n" \
"  - NAND: Operación lógica que da 0 solo si ambos bits de entrada son 1.\n" \
"  - SUMA (Sumador Completo): Suma dos bits y un posible acarreo de entrada, mostrando la suma y el acarreo de salida.\n"



#define TEXTO_PARAMETROS_ENTRADA "\nParámetros de entrada:\n" \
"  - A: Primer bit de entrada (0 o 1). Representa uno de los operandos.\n" \
"  - B: Segundo bit de entrada (0 o 1). Es el otro operando.\n" \
"  - CarryIn: (Solo para la suma) Es el bit de acarreo de entrada, útil cuando se suman varios bits en cadena.\n" \
"\nDurante la ejecución, el simulador te pedirá cada uno de estos valores y te explicará detalladamente el resultado de la operación seleccionada.\n" \
"\n¡Utiliza este simulador para aprender y experimentar cómo funcionan las operaciones básicas de una ALU!\n" \
"\n----------------------------------\n"

#define TEXTO_CARRYIN "\nCarryIn (acarreo de entrada) es un bit que representa si hay un acarreo proveniente de una suma anterior.\n" \
"Por ejemplo, si estás sumando varios bits en cadena (como en números binarios de más de 1 bit), CarryIn permite transferir el acarreo de una suma previa.\n" \
"Si solo quieres sumar dos bits, puedes poner CarryIn = 0.\n" \
"\n¿En qué casos usarías CarryIn = 1?\n" \
"- Cuando la suma anterior (menos significativa) produjo un acarreo.\n" \
"- Por ejemplo, al sumar 1 + 1 en binario, la suma es 0 y el acarreo es 1. Ese 1 se usa como CarryIn en la siguiente posición de bit.\n" \
"- Así, CarryIn = 1 permite simular la suma de números binarios de más de un bit, propagando el acarreo entre posiciones.\n"

#endif // ALU_TEXTOS_H 