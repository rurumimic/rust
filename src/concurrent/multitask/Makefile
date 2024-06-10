CC = clang
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

$(O_FILE): $(ASM_FILE)
	$(CC) $(ASM_FILE) $(CFLAGS) -o $(O_FILE)

$(LIB_FILE): $(O_FILE)
	ar cruUs $(LIB_FILE) $(O_FILE)

# $(TARGET): $(SRC) $(LIB_FILE)
# 	$(CC) $(SRC) $(LDFLAGS) -o $(TARGET)

$(ASM): $(SRC)
	$(CC) -S $(SRC) -o $(ASM)

$(OBJ): $(ASM)
	$(CC) -c $(ASM) -o $(OBJ)

$(TARGET): $(OBJ) $(LIB_FILE)
	$(CC) $(OBJ) $(LDFLAGS) -o $(TARGET)

clean:
	rm -f asm/*.o asm/*.a $(ASM) $(OBJ) $(TARGET)

.PHONY: all clean

