#include <array>
#include <iostream>
#include <thread>

class Foo;

extern "C" {
    void* model__new();
    void model__drop(void* model);
    //void model__init(void* model, void (Foo::*)(const char*), Foo*);
    uint64_t model__stop(void* model);
    //void model__hello(void* model);

    void* model__new_sender(void* model);
    void sender__drop(void* sender);
    uint8_t sender__send_job(void* sender);
}

//class Foo final
//{
//public:
//    Foo() : model(nullptr) { model = model__new("Foo"); }
//    ~Foo() {
//       model__drop(model);
//    }
//
//    void init() {
//       model__init(model, &Foo::send, this);
//
//    }
//    //void hello() { model__hello(model); }
//    void send(const char *mesg) { std::cout << "Foo: " << mesg << std::endl; }
//
//    void *model;
//private:
//    void *sender;
//};

int main(int argc, char *argv[])
{
 //   Foo foo;
 //   foo.init();
    void *model = model__new();

    const int ITERS = 1'000'000;
    std::array<std::thread, 5> threads;
    for (std::thread &t : threads)
    {
        void* sender = model__new_sender(model);
        t = std::thread { [sender = std::move(sender), ITERS]() {
            for (int i = 0; i < ITERS; ++i)
            {
               sender__send_job(sender);
            }
            sender__drop(sender);
        }};
    }
    for (std::thread &t : threads)
    {
        t.join();
    }

    std::cout << "Processed " << model__stop(model) << " jobs" << std::endl;
    model__drop(model);

    return 0;
}