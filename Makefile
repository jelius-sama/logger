all: liblogger.a

# rustc --crate-type=staticlib logger.rs util.rs -o liblogger.a

liblogger.a: util.o logger.rs
	rustc --crate-type=staticlib logger.rs -o liblogger.a
	ar r liblogger.a util.o

util.o: util.c
	gcc -c util.c -DSTRING_IMPLEMENTATION -o util.o

test: test.rs test.c test.go liblogger.a logger.h util.c
	rustc -o test test.rs -L native=. -l static=logger && ./test
	gcc -o test test.c -L. -llogger && ./test
	go build -o test -ldflags="-extldflags '-L. -llogger'" test.go && ./test
