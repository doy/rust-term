#include <termios.h>

int cooked()
{
}

int cbreak()
{
}

int raw()
{
}

int echo(int enabled)
{
    struct termios t;

    if (tcgetattr(0, &t) == -1) {
        return 0;
    }

    if (enabled) {
        t.c_lflag |= ECHO;
    }
    else {
        t.c_lflag &= ~ECHO;
    }

    return tcsetattr(0, TCSANOW, &t) == 0;
}

int crlf(int enabled)
{
}
