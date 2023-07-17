#include <iostream>
#include <chrono>
#include <plotter-cxx/src/lib.rs.h>
#include <future>
#include <thread>
#include <cmath>

int main()
{
    auto start = std::chrono::steady_clock::now();

    rust::Box<CxxSignalGroup> signals = new_signal_group();
    rust::Box<CxxSignal> axis_0 = signals->add_signal("example/sine");

    auto fut = std::async([&] {
        for(;;)
            {
                auto ttp = std::chrono::steady_clock::now() - start;
                double t = static_cast<double>(std::chrono::duration_cast<std::chrono::microseconds>(ttp).count()) / 1000000.0;
                axis_0->push_sample(std::sin(t));
                std::this_thread::sleep_for(std::chrono::milliseconds(5));
            }
    });

    run_gui(std::move(signals));
    
    fut.wait();
}