#include "helper.h"

void signal_handler(int sig) {
    // TODO 5-3: upon receiving SIGTSTP, print "SIGTSTP received"
}

int main() {
    char* ptr;
    long int pid = find_pid("./handler");
    // TODO 5-2: similar to TODO 1-1, register signal handler for SIGTSTP
    if (pid) {
        printf("Send SIGINT to handler (pid: %ld)\n", pid);
        // TODO 4: send SIGINT signal to pid using "kill" function
    }
    return 0;	
}