#include <iostream>

#include "..\generated\book_generated.h"

#include "graph/edge.h"
#include "graph/vector.h"

#include "flatbufferutils/createBook.h"
#include "flatbufferutils/processBook.h"

#include "aeron/settings.h"
#include "aeron/subscriber.h"
#include "FragmentAssembler.h"

// boost::stable_vector
/*
   SPMC Queue

    struct SPSCQueue {
        alignas(64) std::atomic<uint64_t> _writeIndex;
        alignas(64) std::atomic<uint64_t> _readIndex;
        alignas(64) uint8_t _data[0];
    }
*/

// Push metrics to SPSC Queues

// https://github.com/cameron314/concurrentqueue
// https://github.com/rigtorp/SPSCQueue


// Fast-Log: https://github.com/maciekgajewski/fast-log
// Variadic Logging: https://github.com/carlcook/variadicLogging

// Compiler Switches to try
// -ffast-math ( Be careful )
// -march, -mtune

// http://www.reedbeta.com/blog/data-oriented-hash-table

// Userspace Networking such as OpenOnload

// SG14...
// https://github.com/WG21-SG14/SG14/tree/master/SG14

GraphImpl::Vector setupConsumerNode() {
    return GraphImpl::Vector( []() { 
        std::cout << "GraphImpl::Vector::Receive" << std::endl;
    });
}

GraphImpl::Vector setupProducerNode() {
    return GraphImpl::Vector( []() { 
        std::vector<Graph::FlatBufferUtils::BufferT> buf =
            Graph::FlatBufferUtils::AsBuffer();

        std::cout << "GraphImpl::Vector::Produce:" << buf.size() << std::endl;
    });
}

aeron::fragment_handler_t printStringMessage()
{
    return [&](const aeron::AtomicBuffer &buffer, aeron::util::index_t offset, aeron::util::index_t length, const aeron::Header &header)
    {
        std::cout
            << "Message to stream " << header.streamId() << " from session " << header.sessionId()
            << "(" << length << "@" << offset << ") <<"
            << std::string(reinterpret_cast<const char *>(buffer.buffer()) + offset, static_cast<std::size_t>(length))
            << ">>" << std::endl;
    };
}

int main(int argc, char** argv) {    const auto consumerNode = setupConsumerNode(); 

    std::atomic<bool> isRunning{true};

    const auto producerNode = setupProducerNode(); 
    const GraphImpl::Edge edge(producerNode, consumerNode);

    // producerNode.execute();

    std::vector<Graph::FlatBufferUtils::BufferT> buf =
        Graph::FlatBufferUtils::AsBuffer();

    Graph::FlatBufferUtils::FromBuffer(buf);

     
    auto publisher = Graph::Aeron::Publisher::Create(Graph::Aeron::Settings());
    // publisher->offer(srcBuffer, 0, messageLen);

    // 
    auto subscriber = Graph::Aeron::Subscriber::Create(Graph::Aeron::Settings());

    aeron::FragmentAssembler fragmentAssembler(printStringMessage());
    aeron::fragment_handler_t handler = fragmentAssembler.handler();
    Graph::Aeron::Subscriber::handle(*subscriber, [&isRunning]() -> bool { return isRunning; }, handler );

    // auto agent = Graph::Aeron::Subscriber::MyAgent();
    // AgentRunner.startOnThread(runner);

    std::cout << "Hello from graph main" << std::endl;
    
    return 1;
}
