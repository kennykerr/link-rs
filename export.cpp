#include <stdint.h>

struct complex
{
    int32_t a;
    int32_t b;
};

struct more_complex
{
    int64_t a;
    int64_t b;
    int64_t c;
};

extern "C" __declspec(dllexport) int64_t __stdcall return_simple()
{
    return 123;
}

extern "C" __declspec(dllexport) complex __stdcall return_complex()
{
    return { 123, 456 };
}

extern "C" __declspec(dllexport) more_complex __stdcall return_more_complex()
{
    return { 123, 456, 789 };
}
