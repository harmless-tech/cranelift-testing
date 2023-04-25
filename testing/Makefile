CC = gcc
CFLAGS= -g -pedantic-errors -Wall -fPIC -fstack-protector-all -Werror -Wshadow

all: main.x

main.x: main.o cr.o
	$(CC) cr.o main.o -o main.x

main.o: main.c
	$(CC) $(CFLAGS) -c main.c -o main.o

clean:
	rm -f main.o main.x
