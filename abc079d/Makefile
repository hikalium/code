CPPFLAG=-Wall -Wpedantic -std=c++1y

a.out : a.cpp Makefile
	g++ $(CPPFLAG) a.cpp

copy : a.cpp Makefile
	cat a.cpp | pbcopy

clean:
	-rm a.out
