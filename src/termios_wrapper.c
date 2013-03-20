#include <stdlib.h>
#include <errno.h>
#include <termios.h>
#include <sys/ioctl.h>

/* very simplistic, ignores a lot of the settings that i don't understand,
 * patches welcome */

int cooked()
{
    struct termios t;

    if (tcgetattr(0, &t) == -1) {
        return errno;
    }

    t.c_lflag |= (ICANON | ISIG | IEXTEN);
    t.c_iflag |= (IXON | BRKINT);

    return tcsetattr(0, TCSANOW, &t) == 0 ? 0 : errno;
}

int cbreak()
{
    struct termios t;

    if (tcgetattr(0, &t) == -1) {
        return errno;
    }

    t.c_lflag |= ISIG;
    t.c_lflag &= ~(ICANON | IEXTEN);
    t.c_iflag |= (IXON | BRKINT);

    return tcsetattr(0, TCSANOW, &t) == 0 ? 0 : errno;
}

int raw()
{
    struct termios t;

    if (tcgetattr(0, &t) == -1) {
        return errno;
    }

    t.c_lflag &= ~(ICANON | ISIG | IEXTEN);
    t.c_iflag &= ~(IXON | BRKINT);

    return tcsetattr(0, TCSANOW, &t) == 0 ? 0 : errno;
}

int echo(int enabled)
{
    struct termios t;

    if (tcgetattr(0, &t) == -1) {
        return errno;
    }

    if (enabled) {
        t.c_lflag |= ECHO;
    }
    else {
        t.c_lflag &= ~ECHO;
    }

    return tcsetattr(0, TCSANOW, &t) == 0 ? 0 : errno;
}

struct termios *get()
{
    struct termios *t;

    t = malloc(sizeof(struct termios));
    if (tcgetattr(0, t) == -1) {
        return NULL;
    }

    return t;
}

void set(struct termios *t)
{
    if (t == NULL) {
        return;
    }

    tcsetattr(0, TCSANOW, t);
    free(t);
}

void size(unsigned int *rows, unsigned int *cols)
{
    struct winsize ws;
    ioctl(0, TIOCGWINSZ, &ws);
    *rows = ws.ws_row;
    *cols = ws.ws_col;
}
