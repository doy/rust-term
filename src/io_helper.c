#include <stdlib.h>
#include <sys/select.h>

int timed_read(int timeout)
{
    char byte;

    if (timeout >= 0) {
        fd_set readfds;
        struct timeval t;

        FD_ZERO(&readfds);
        FD_SET(0, &readfds);

        t.tv_sec  = timeout / 1000000;
        t.tv_usec = timeout % 1000000;

        if (select(1, &readfds, NULL, NULL, &t) == 1) {
            if (read(0, &byte, 1) == 1) {
                return byte;
            }
            else {
                return -1;
            }
        }
        else {
            return -1;
        }
    }
    else {
        if (read(0, &byte, 1) == 1) {
            return byte;
        }
        else {
            return -1;
        }
    }
}
