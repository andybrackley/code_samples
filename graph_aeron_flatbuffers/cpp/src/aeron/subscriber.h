#pragma once

#include <Aeron.h>

// Aeron: 
//   Wiki: https://aeron.io/docs/
//   GitHub: https://github.com/real-logic/aeron
//   CppGuide: https://github.com/real-logic/Aeron/wiki/Cpp-Programming-Guide

/*
void setupAeronPub() {
    aeron::concurrent::AtomicBuffer srcBuffer;
    aeron::Publication pub;
    pub.offer(srcBuffer);
}

void setupAeronSub() {
    auto context = aeron::Context();
    auto aeron = aeron::Aeron::connect(context);

}

*/

/* Code Samples 

Subscribe to endpoint:

Subscription subscription 
    = aeron.addSubscription("aeron:udp?endpoint=192.168.0.1:12345", 1);

Duty Cycles:

EpochClock clock = new SystemEpochClock();

while (true)
{
    Command command = adaptInputBuffer();
    routeToAppropriateBusinessLogic(command);
}

*/

// Argona Agents
//    Full Example: https://aeron.io/docs/aeron-cookbook/ipc/

namespace Graph {
namespace Aeron {
namespace Subscriber {


struct AgentImpl {};
// struct IdleStrategy {};


using MyAgent = aeron::AgentRunner<AgentImpl, aeron::concurrent::SleepingIdleStrategy>;

}

struct BufferStrategyPuts {
    void toBuffer(uint32_t size, void* buffer, uint32_t offset) {}
};

}
}
