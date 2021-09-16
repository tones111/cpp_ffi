#include <iostream>

class Foo;

extern "C" {
    void* model__new(const char* name);
    void model__drop(void* model);
    void model__init(void* model, void (Foo::*)(const char*), Foo*);
    void model__hello(void* model);
}

class Foo final
{
public:
    Foo() : model(nullptr) { model = model__new("Foo"); }
    ~Foo() { model__drop(model); }

    void init() { model__init(model, &Foo::send, this); }
    void hello() { model__hello(model); }
    void send(const char *mesg) { std::cout << "Foo: " << mesg << std::endl; }

private:
    void *model;
};

int main(int argc, char *argv[])
{
    Foo foo;
    foo.init();
    foo.hello();

    return 0;
}