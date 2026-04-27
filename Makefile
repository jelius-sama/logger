all: liblogger.a

liblogger.a: logger.rs
	rustc --crate-type=staticlib logger.rs -o liblogger.a

test: test.rs test.c test.go liblogger.a logger.h
	@rustc -o test test.rs -L native=. -l static=logger && ./test
	@gcc -o test test.c -L. -llogger && ./test
	@go build -o test -ldflags="-extldflags '-L. -llogger'" test.go && ./test
	@rm ./test
