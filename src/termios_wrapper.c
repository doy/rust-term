#include <termios.h>

/* very simplistic, ignores a lot of the settings that i don't understand,
 * patches welcome */

int cooked()
{
    struct termios t;

    if (tcgetattr(0, &t) == -1) {
        return 0;
    }

    t.c_lflag |= (ICANON | ISIG | IEXTEN);
    t.c_iflag |= (IXON | BRKINT);

    return tcsetattr(0, TCSANOW, &t) == 0;
}

int cbreak()
{
    struct termios t;

    if (tcgetattr(0, &t) == -1) {
        return 0;
    }

    t.c_lflag |= ISIG;
    t.c_lflag &= ~(ICANON | IEXTEN);
    t.c_iflag |= (IXON | BRKINT);

    return tcsetattr(0, TCSANOW, &t) == 0;
}

int raw()
{
    struct termios t;

    if (tcgetattr(0, &t) == -1) {
        return 0;
    }

    t.c_lflag &= ~(ICANON | ISIG | IEXTEN);
    t.c_iflag &= ~(IXON | BRKINT);

    return tcsetattr(0, TCSANOW, &t) == 0;
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
