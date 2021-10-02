#include <array>
#include <iostream>
#include <thread>

class Foo;

extern "C" {
    void* model__new();
    void model__drop(void* model);
    void model__serve(void* model);
    void model__stop(void* model);

    void* model__new_sender(void* model);
    void sender__drop(void* sender);
    uint8_t sender__send_job(void* sender);
}

int main(int argc, char *argv[])
{
    void *model = model__new();
    model__serve(model);

    void* sender1 = model__new_sender(model);
    sender__send_job(sender1);
    sender__drop(sender1);

    model__stop(model);
    model__drop(model);

    return 0;
}