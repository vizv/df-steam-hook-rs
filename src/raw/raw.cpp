#include <string>
#include <cstring>

using namespace std;

extern "C"
{
    void *string_create()
    {
        string *s = new string();
        return reinterpret_cast<void *>(s);
    }

    void *string_create_n_chars(size_t len, char ch)
    {
        string *s = new string(len, ch);
        return reinterpret_cast<void *>(s);
    }

    void string_delete(void *ptr)
    {
        string *s = reinterpret_cast<string *>(ptr);
        delete s;
    }

    size_t string_len(void *ptr)
    {
        string *s = reinterpret_cast<string *>(ptr);
        return s->size();
    }

    const char *string_to_str(void *ptr)
    {
        string *s = reinterpret_cast<string *>(ptr);
        return s->c_str();
    }
}
