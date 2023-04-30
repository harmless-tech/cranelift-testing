CC = gcc
CFLAGS= -g -pedantic-errors -Wall -fPIC -fstack-protector-all -Werror -Wshadow

all: main.x

main.x: main.o wrapper.o cr.o
	$(CC) cr.o wrapper.o main.o -o main.x

main.o: main.c
	$(CC) $(CFLAGS) -c main.c -o main.o

wrapper.o: wrapper.c
	$(CC) $(CFLAGS) -c wrapper.c -o wrapper.o

clean:
	rm -f wrapper.o main.o main.x
