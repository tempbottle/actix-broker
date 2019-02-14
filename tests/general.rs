#[macro_use]
extern crate actix;
extern crate actix_broker;

use actix::prelude::*;
use actix_broker::{BrokerIssue, BrokerSubscribe};

use std::time::Duration;

#[derive(Clone, Message)]
struct TestMessageOne(u8);

struct TestActorOne;

struct TestActorTwo;

impl Actor for TestActorOne {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_sync::<TestMessageOne>(ctx);
        ctx.run_later(Duration::from_millis(250), |a, _| {
            a.issue_async(TestMessageOne(255))
        });
    }
}

impl Actor for TestActorTwo {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_sync::<TestMessageOne>(ctx);
    }
}

impl Handler<TestMessageOne> for TestActorOne {
    type Result = ();

    fn handle(&mut self, msg: TestMessageOne, _ctx: &mut Self::Context) {
        assert_eq!(msg.0, 125);
        System::current().stop();
    }
}

impl Handler<TestMessageOne> for TestActorTwo {
    type Result = ();

    fn handle(&mut self, msg: TestMessageOne, _ctx: &mut Self::Context) {
        assert_eq!(msg.0, 255);
        self.issue_async(TestMessageOne(125));
    }
}

#[test]
fn it_all_works() {
    System::run(|| {
        TestActorOne.start();
        TestActorTwo.start();
    });
}
