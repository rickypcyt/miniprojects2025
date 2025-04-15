arr = [6,3,2,4,7,8,5]
print(arr)
posiciones = [0,1,2,3,4,5,6,7,8,9]
print(posiciones)

#Quiero hacer que pueda encontrar la posicion de tal valor tipo si yo digo que x es igual a 4 deberia de decirme que esta en la posicion 3 o sea empezando desde 0 el contador o 4 si se empieza desde uno.

x = int(input("Posicion del array:"))

if x > 0 and x <= len(arr):
    print (("En la posicion x esta el num:"),{arr[x]})

#Tambien quiero que me imprima el array sin ordenar y el ordenado.
#Vamos a hacerlo con insertionsort que es bien cool lo que hace es que va iterando y va moviendo elementos segun los anteriores digamos tienes 3421 ve si 3 es mayor a 4 y ve que no entonces hace nada de ahi 4 y 2, el dos es menos que 4 asi que lo mueve a la izquierda, como lo mueve a la izquierda entonces ahora compara 3 y 2, 2 es menor que 3 asi que lo mueve a la izquierda. Se empieza a veces por el 2do elemento porque el primero no teine nada a la izquierda. 

def insertion_sort(arr):
    for i in range(1, len(arr)):  # Empezamos desde 1 porque el primer elemento ya está "ordenado"
        actual = arr[i]  # Elemento que queremos insertar
        anterior = i - 1  # El índice del elemento anterior

        # Mueve los elementos mayores que `actual` una posición a la derecha
        #lo de mayor o igual a 0 es solo para que no te salgas del array. anterior comienza como i menos 1 
        while anterior >= 0 and arr[anterior] > actual:
            arr[anterior + 1] = arr[anterior]  # Desplaza el elemento a la derecha
            anterior = anterior - 1  # Decrementa el índice

        # Coloca `actual` en su posición correcta
        arr[anterior + 1] = actual

    return arr  # Devuelve el array ordenado


# Ejemplo de uso:
arr_ordenado = insertion_sort(arr)  # Llamamos a la función de ordenación
print(arr_ordenado)  # Imprime el array ordenado
