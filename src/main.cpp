#include <iostream>
#include "rust-lib/src/lib.rs.h"

int main()
{
    std::cout << "Hello rust! " << run_plotter() << "\n";
}