use amenable_kani::{KaniChannel, KaniRecvError, KaniSendError};

#[test]
fn unbounded_send_then_recv_delivers_the_value() -> miette::Result<()> {
    let mut channel = KaniChannel::unbounded();
    channel
        .send(1)
        .map_err(|error| miette::miette!("{error:?}"))?;
    assert_eq!(channel.recv(), Ok(1));
    Ok(())
}

#[test]
fn unbounded_channel_preserves_fifo_order() -> miette::Result<()> {
    let mut channel = KaniChannel::unbounded();
    channel
        .send(1)
        .map_err(|error| miette::miette!("{error:?}"))?;
    channel
        .send(2)
        .map_err(|error| miette::miette!("{error:?}"))?;
    assert_eq!(channel.recv(), Ok(1));
    assert_eq!(channel.recv(), Ok(2));
    Ok(())
}

#[test]
fn recv_fails_once_empty_and_sender_dropped() {
    let mut channel: KaniChannel<i32> = KaniChannel::unbounded();
    channel.drop_sender();
    assert_eq!(channel.recv(), Err(KaniRecvError::Disconnected));
}

#[test]
fn try_recv_distinguishes_empty_from_disconnected() {
    let mut channel: KaniChannel<i32> = KaniChannel::unbounded();
    assert_eq!(channel.try_recv(), Err(KaniRecvError::Empty));

    channel.drop_sender();
    assert_eq!(channel.try_recv(), Err(KaniRecvError::Disconnected));
}

#[test]
fn send_fails_and_recovers_the_value_once_receiver_dropped() {
    let mut channel = KaniChannel::unbounded();
    channel.drop_receiver();
    assert_eq!(channel.send(7), Err(KaniSendError::Disconnected(7)));
}

#[test]
fn bounded_zero_capacity_try_send_fails_full_and_recovers_the_value() {
    let mut channel = KaniChannel::bounded(0);
    assert_eq!(channel.try_send(7), Err(KaniSendError::Full(7)));
}

#[test]
fn bounded_channel_accepts_sends_up_to_capacity() -> miette::Result<()> {
    let mut channel = KaniChannel::bounded(1);
    channel
        .send(1)
        .map_err(|error| miette::miette!("{error:?}"))?;
    assert_eq!(channel.try_send(2), Err(KaniSendError::Full(2)));
    assert_eq!(channel.recv(), Ok(1));
    channel
        .send(2)
        .map_err(|error| miette::miette!("{error:?}"))?;
    Ok(())
}
