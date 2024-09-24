#include <iostream>
#include <thread>

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

aeron::fragment_handler_t printStringMessage(std::atomic<bool>& signal)
{
    return [&](const aeron::AtomicBuffer &buffer, aeron::util::index_t offset, aeron::util::index_t length, const aeron::Header &header)
    {
        std::vector<uint8_t> bookBuf;

        bookBuf.reserve(length);
        bookBuf.insert(bookBuf.end(), buffer.buffer() + offset, buffer.buffer() + length + offset);

        Graph::FlatBufferUtils::FromBuffer(bookBuf);
        signal.store(false);
    };
}

void runSubscriptionHandler(std::atomic<bool>& signal, aeron::Subscription& subscriber) {
    aeron::FragmentAssembler fragmentAssembler(printStringMessage(signal));
    aeron::fragment_handler_t handler = fragmentAssembler.handler();    
    Graph::Aeron::Subscriber::handle(subscriber, [&]() -> bool { return signal.load(); } , handler );
}

typedef std::array<std::uint8_t, 256> buffer_t;

void runPublisher(std::atomic<bool>& signal, aeron::Publication& publisher) {
    AERON_DECL_ALIGNED(buffer_t buffer, 16);
    aeron::concurrent::AtomicBuffer srcBuffer(&buffer[0], buffer.size());

    const auto bookBuffer = Graph::FlatBufferUtils::AsBuffer();

    srcBuffer.putBytes(0, bookBuffer.data(), bookBuffer.size());
    const std::int64_t result = publisher.offer(srcBuffer, 0, bookBuffer.size());

    // Might be able to do the above by directly setting memory rather than having to do a publish
    // See the samples: https://github.com/real-logic/aeron/blob/master/aeron-samples/src/main/cpp/StreamingPublisher.cpp
    // srcBuffer.setMemory(0, settings.messageLength, 0);

    if(result > 0) {
        std::cout << "Sent: " << result << std::endl;
    } else {
        std::cout << "Failed with code: " << result << " pub connection: " << publisher.isConnected() << " status: " << publisher.channelStatus() << std::endl;
        signal.store(false);
    }
}

int main(int argc, char** argv) {    
    const auto consumerNode = setupConsumerNode(); 
    std::atomic<bool> isRunning{true};

    const auto producerNode = setupProducerNode(); 
    const GraphImpl::Edge edge(producerNode, consumerNode);

    // producerNode.execute();

    std::vector<Graph::FlatBufferUtils::BufferT> buf =
        Graph::FlatBufferUtils::AsBuffer();

    Graph::FlatBufferUtils::FromBuffer(buf);

     
    Graph::Aeron::Settings settings("");
    auto connection = Graph::Aeron::Connection::Connect(settings);

    auto publisher = Graph::Aeron::Publisher::Create(*connection, settings);
    Graph::Aeron::dumpConnectionStatus(*publisher);

    auto subscriber = Graph::Aeron::Subscriber::Create(*connection, settings);

    std::thread tSub(runSubscriptionHandler, std::ref(isRunning), std::ref(*subscriber));
    std::thread tPub(runPublisher, std::ref(isRunning), std::ref(*publisher));

    std::cout << "Waiting for subscriber to close" << std::endl;
    tPub.join();
    tSub.join();

    std::cout << "Publish/Subscribe has completed.  Closing...." << std::endl;
    return 1;
}
