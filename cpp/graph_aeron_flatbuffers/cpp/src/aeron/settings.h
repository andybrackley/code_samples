#pragma once

#include "Aeron.h"

#include <cstdint>

namespace Graph {
namespace Aeron {
    struct Settings {
        const std::string channel = "1";
        const std::int32_t streamId = 1;
    };

    aeron::Context setupContext() {
        aeron::Context context;
        return context;
    }

    std::shared_ptr<aeron::Aeron> Connect(aeron::Context& context) {
        std::shared_ptr<aeron::Aeron> connection = aeron::Aeron::connect(context);
        return connection;
    }

    namespace Publisher {
        std::shared_ptr<aeron::Publication> GetById(aeron::Aeron& connection, std::int32_t pubId) {
            std::shared_ptr<aeron::Publication> pub = connection.findPublication(pubId);
            while(!pub) {
                std::this_thread::yield();
                pub = connection.findPublication(pubId);
            }

            return pub;
        }

        std::shared_ptr<aeron::Publication> SetupAndGet(aeron::Aeron& connection, const Settings& settings) {
            std::int64_t id = connection.addPublication(settings.channel, settings.streamId);
            return GetById(connection, id);
        }

        std::shared_ptr<aeron::Publication> Create(const Settings& settings) {
            auto context = setupContext();
            auto connection = Connect(context);
            auto publisher = SetupAndGet(*connection, settings);
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

            return sub;
        }

        std::shared_ptr<aeron::Subscription> SetupAndGet(aeron::Aeron& connection, const Settings& settings) {
            std::int64_t id = connection.addSubscription(settings.channel, settings.streamId);
            return GetById(connection, id);
        }

        std::shared_ptr<aeron::Subscription> Create(const Settings& settings) {
            auto context = setupContext();
            auto connection = Connect(context);
            auto subscriber = SetupAndGet(*connection, settings);
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