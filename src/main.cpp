#include <iostream>

extern "C" {
    void* model__new(const char* name);
    void model__drop(void* model);
    void model__init(void* model);
    void model__hello(void* model);
}

class Foo final
{
public:
    Foo() : model(nullptr) { model = model__new("Foo"); }
    ~Foo() { model__drop(model); }

    void init() { model__init(model); }
    void hello() { model__hello(model); }

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