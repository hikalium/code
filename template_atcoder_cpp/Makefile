CXX=clang++
CXXFLAG=-std=c++14 -stdlib=libc++ -O2

%.bin : %.cc Makefile
	$(CXX) $(CXXFLAG) -o $@ $*.cc

clean:
	-rm *.bin
