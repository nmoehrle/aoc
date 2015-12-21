#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>

typedef enum {false, true} bool;

static bool incstr(char * s, int n) {
    while (n >= 0) {
        if (s[n] >= 'z') {
            s[n] = 'a';
            n -= 1;
        } else {
            s[n] += 1;
            return true;
        }
    }
    return false;
}

static bool validpw(const char * s) {
    const char *e = s;
    
    while(*e++) {
        if (*e == 'i' || *e == 'o' || *e == 'l') {
            return false;
        }
    }
    const int l = e - s - 1;
    
    int tuples = 0, seq = 0;
    
    for(int i = 0; i < l - 2; ++i) {
        if (s[i] + 1 == s[i + 1]
            && s[i + 1] + 1 == s[i + 2]) {
            ++seq;
        }
    }

    for(int i = 0; i < l - 1; ++i) {
        if (s[i] == s[i + 1]) {
            ++tuples; ++i;
        }
    }
    return tuples >= 2 && seq >= 1;
}

static int l_newpw (lua_State *L) {
    const char *s = luaL_checkstring(L, 1);
    char ns[9];
    for (int i = 0; i < 8; ++i) {
        ns[i] = s[i];
    }
    ns[8] = 0;

    while (!validpw(ns)) {
        if (!incstr(ns, 7)) {
            return 0;
        }
    }

    lua_pushstring(L, ns);
    return 1;
}

static const struct luaL_Reg pwlib [] = {
    {"newpw", l_newpw},
    {NULL, NULL}
};

int luaopen_pwlib (lua_State *L) {
    luaL_openlib(L, "pwlib", pwlib, 0);
    return 1;
}
