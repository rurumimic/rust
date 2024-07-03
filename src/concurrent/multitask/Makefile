CC = gcc
ASM_FILE = asm/context.S
O_FILE = asm/context.o
LIB_FILE = asm/libcontext.a
CFLAGS = -c -fPIC -ggdb
LDFLAGS = -Lasm -lcontext

TARGET = main
SRC = main.c
ASM = main.s
OBJ = main.o

all: $(TARGET)

# compile the assembly file into an object file
$(O_FILE): $(ASM_FILE)
	$(CC) $(ASM_FILE) $(CFLAGS) -o $(O_FILE)

# create a static library from the object file
$(LIB_FILE): $(O_FILE)
	ar cruUs $(LIB_FILE) $(O_FILE)

# compile the source file into an object file
# $(TARGET): $(SRC) $(LIB_FILE)
# 	$(CC) $(SRC) $(LDFLAGS) -o $(TARGET)

# compile the source file into an assembly file
$(ASM): $(SRC)
	$(CC) -S $(SRC) -o $(ASM)

# compile the assembly file into an object file
$(OBJ): $(ASM)
	$(CC) -c $(ASM) -o $(OBJ)

# link the object file with the static library
$(TARGET): $(OBJ) $(LIB_FILE)
	$(CC) $(OBJ) $(LDFLAGS) -o $(TARGET)

# remove all generated files
clean:
	rm -f asm/*.o asm/*.a $(ASM) $(OBJ) $(TARGET)

# declare the targets that are not files
.PHONY: all clean

