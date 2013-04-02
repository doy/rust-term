#include <curses.h>
#include <term.h>

char *tiparm1(const char *name, int p1)
{
    return tiparm(name, p1);
}

char *tiparm2(const char *name, int p1, int p2)
{
    return tiparm(name, p1, p2);
}
