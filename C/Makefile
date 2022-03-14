TARGETS=sender handler

all: $(TARGETS) 

sender: sender.c helper.c
	gcc sender.c helper.c -o sender

handler: handler.c helper.c
	gcc handler.c helper.c -o handler

clean:
	rm -f $(TARGETS)