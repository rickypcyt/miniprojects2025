CC=gcc
CFLAGS=-I./clay/include -Wall -Wextra -std=c99
LDFLAGS=-L./clay/build -lclay -lwayland-client -lwayland-server -lwlroots -lpixman-1 -lm

SRC=src/main.c
BIN=showinfo

all:
	$(CC) $(CFLAGS) $(SRC) -o $(BIN) $(LDFLAGS)

run: all
	./$(BIN)

clean:
	rm -f $(BIN)

