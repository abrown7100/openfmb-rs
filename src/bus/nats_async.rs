// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use async_trait::async_trait;
use bytes::BytesMut;
use futures::StreamExt;
use std::marker::PhantomData;

use super::Closable;

/// Nats Message Bus
#[derive(Debug, Clone)]
pub struct NatsAsyncBus<E: MessageEncoding> {
    conn: async_nats::Client,
    encoding: PhantomData<E>,
}

fn topic_to_subject<S: AsRef<str>, T: Topic<S>>(topic: T) -> String {
    let prefix_match = topic.prefix_match();
    let mut subject = String::with_capacity(128);
    topic.for_each(|lvl| match lvl {
        TopicLevel::Exact(val) => {
            subject.push_str(val.as_ref());
            subject.push('.');
        }
        TopicLevel::WildCard => subject.push_str("*."),
    });

    if prefix_match {
        subject.push('>');
    } else {
        subject.pop();
    }
    subject
}

fn topic_level<'a>(level: &'a str) -> TopicLevel<&'a str> {
    match level {
        "*" => TopicLevel::WildCard,
        val => TopicLevel::Exact(val),
    }
}

#[derive(Debug)]
pub struct SubjectIter<'s> {
    iter: std::iter::Map<std::str::Split<'s, char>, for<'a> fn(&'a str) -> TopicLevel<&'a str>>,
}

impl<'s> Iterator for SubjectIter<'s> {
    type Item = TopicLevel<&'s str>;
    fn next(&mut self) -> Option<TopicLevel<&'s str>> {
        self.iter.next()
    }
}

impl<'a> Topic<&'a str> for SubjectIter<'a> {
    fn prefix_match(&self) -> bool {
        false
    }
}

fn subject_to_topic<'a>(subject: &'a str) -> SubjectIter<'a> {
    SubjectIter {
        iter: subject.split('.').map(topic_level),
    }
}

#[async_trait]
impl<E, M> Subscriber<M> for NatsAsyncBus<E>
where
    E: 'static + MessageEncoding + Send,
    M: 'static + Message<E> + Send,
{
    async fn subscribe<S: AsRef<str>, T: Topic<S>>(
        &mut self,
        topic: T,
    ) -> Result<Subscription<M>, SubscribeError> {
        //debug!("subscribing to {:?}", topic);
        let subject: String = topic_to_subject(topic);

        let subscriber = self
            .conn
            .subscribe(subject)
            .await
            .map_err(|err| SubscribeError::BusError(Box::new(err)))?;

        Ok(Box::pin(subscriber.map(|msg| {
            let topic = subject_to_topic(&msg.subject);
            let data: &[u8] = &msg.payload;

            M::decode(topic, data).map_err(|err| SubscriptionError::DecodeError(Box::new(err)))
        })))
    }
}

#[async_trait]
impl<E, M> Publisher<M> for NatsAsyncBus<E>
where
    E: 'static + MessageEncoding + Send,
    M: 'static + Message<E> + Send,
{
    async fn publish<S: AsRef<str>, T: Topic<S>>(&mut self, topic: T, msg: M) -> PublishResult<()> {
        let mut buf = BytesMut::new();
        msg.encode(&mut buf)
            .map_err(|err| PublishError::EncodeError(Box::new(err)))?;
        let subject: String = topic_to_subject(topic);

        let buf = buf.freeze();

        Ok(self
            .conn
            .publish(subject, buf)
            .await
            .map_err(|err| PublishError::BusError(Box::new(err)))?)
    }
}

/// A nats connection implements the MessageBus trait
impl<E, M> MessageBus<M> for NatsAsyncBus<E>
where
    E: 'static + MessageEncoding + Send,
    M: 'static + Send + Message<E>,
{
}

impl<E> Closable<E> for NatsAsyncBus<E>
where
    E: 'static + MessageEncoding + Send,
{
    fn close(&self) {
        // TODO close
    }
}

impl<E> NatsAsyncBus<E>
where
    E: 'static + MessageEncoding + Send,
{
    pub fn new(conn: async_nats::Client) -> NatsAsyncBus<E> {
        NatsAsyncBus {
            conn,
            encoding: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Topic, TopicLevel};

    use super::{subject_to_topic, topic_to_subject};

    #[test]
    fn test_subject_conversion() {
        assert_eq!(topic_to_subject(subject_to_topic("x.y.*")), "x.y.*");
        assert_eq!(topic_to_subject(subject_to_topic("x.y.z")), "x.y.z");
        assert_eq!(topic_to_subject(subject_to_topic("x")), "x");
        assert_eq!(subject_to_topic("x.y").prefix_match(), false);
        let parts0: Vec<TopicLevel<&str>> = subject_to_topic("x.y").collect();
        assert_eq!(parts0, vec![TopicLevel::Exact("x"), TopicLevel::Exact("y")]);
        let parts1: Vec<TopicLevel<&str>> = subject_to_topic("x.*").collect();
        assert_eq!(parts1, vec![TopicLevel::Exact("x"), TopicLevel::WildCard]);
    }
}
