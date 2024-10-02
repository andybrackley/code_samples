#pragma once

#include "Aeron.h"

#include <cstdint>

namespace Graph {
namespace Aeron {
    struct Settings {
        const std::string channel = "aeron:ipc";
        const std::int32_t streamId = 1;
        const std::string dirPrefix;

        Settings(const std::string& dir) : dirPrefix(dir) {}
    };

    template<typename ConT>
    void dumpConnectionStatus(const ConT& pub) {
        const int channelStatus = pub.channelStatus();

        std::cout << "Publication channel status: " << channelStatus << "(id=" << pub.channelStatusId() << ") "
            << (channelStatus == aeron::ChannelEndpointStatus::CHANNEL_ENDPOINT_ACTIVE
                    ? "ACTIVE"
                    : std::to_string(channelStatus))
            << std::endl;
    }

    namespace Connection {
        aeron::Context createContext(const Settings& settings) {
            aeron::Context context;
            if(!settings.dirPrefix.empty()) {
                context.aeronDir(settings.dirPrefix);
            }

            std::cout << "Using cnc::file:" << context.cncFileName() << std::endl;
            return context;
        }

        std::shared_ptr<aeron::Aeron> Connect(const Settings& settings) {
            auto context = createContext(settings);
            std::shared_ptr<aeron::Aeron> connection = aeron::Aeron::connect(context);
            return connection;
        }
    }

    namespace Publisher {
        std::shared_ptr<aeron::Publication> GetById(aeron::Aeron& connection, std::int32_t pubId) {
            std::shared_ptr<aeron::Publication> pub = connection.findPublication(pubId);
            while(!pub) {
                std::this_thread::yield();
                pub = connection.findPublication(pubId);
            }

            dumpConnectionStatus(*pub);
            return pub;
        }

        std::shared_ptr<aeron::Publication> SetupAndGet(aeron::Aeron& connection, const Settings& settings) {
            std::int64_t id = connection.addPublication(settings.channel, settings.streamId);
            return GetById(connection, id);
        }

        std::shared_ptr<aeron::Publication> Create(aeron::Aeron& connection, const Settings& settings) {
            auto publisher = SetupAndGet(connection, settings);
            return publisher;
        }
    }

    namespace Subscriber {
        constexpr std::chrono::duration<long, std::milli> IDLE_SLEEP_MS(1);
        constexpr int FRAGMENTS_LIMIT = 10;

        std::shared_ptr<aeron::Subscription> GetById(aeron::Aeron& connection, std::int32_t subId) {
            std::shared_ptr<aeron::Subscription> sub = connection.findSubscription(subId);
            while(!sub) {
                std::this_thread::yield();
                sub = connection.findSubscription(subId);
            }

            dumpConnectionStatus(*sub);
            return sub;
        }

        std::shared_ptr<aeron::Subscription> SetupAndGet(aeron::Aeron& connection, const Settings& settings) {
            std::int64_t id = connection.addSubscription(settings.channel, settings.streamId);
            return GetById(connection, id);
        }

        std::shared_ptr<aeron::Subscription> Create(aeron::Aeron& connection, Settings& settings) {
            auto subscriber = SetupAndGet(connection, settings);
            return subscriber;
        }

        void handle(aeron::Subscription& subscription, std::function<bool()> isRunning, aeron::fragment_handler_t& handler ) {
            aeron::SleepingIdleStrategy idleStrategy(IDLE_SLEEP_MS);
            
            while(isRunning()) {
                const int fragmentsRead = subscription.poll(handler, FRAGMENTS_LIMIT);
                idleStrategy.idle(fragmentsRead);
            }
        }
    }
}
}