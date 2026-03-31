use fh_telemetry::{Listener, Result};
use fh_telemetry::listener::StopSignal;
use tokio::net::ToSocketAddrs;
use tokio::sync::{mpsc, oneshot};

#[derive(Default)]
pub(crate) struct EngineData {
    pub idle_rpm: f32,
    pub max_rpm: f32,
    pub power: Vec<(f32, f32)>,
    pub torque: Vec<(f32, f32)>,
    pub peak: f32,
}

pub(crate) async fn recv<A: ToSocketAddrs>(addr: A) -> Result<EngineData> {
    let listener = Listener::new(addr).await?.listen().await?;

    let (tx, mut rx) = mpsc::channel(1024);
    let (stop_tx, stop_rx) = oneshot::channel();

    tokio::spawn(async move {
        if listener.recv(tx, stop_rx).await.is_err() {
            stop_tx.send(StopSignal).unwrap();
        }
    });

    let mut data = EngineData::default();

    while let Some(packet) = rx.recv().await {
        if packet.accel > 0 {
            if packet.power < 0.0 {
                data.idle_rpm = packet.engine_idle_rpm;
                data.max_rpm = packet.engine_max_rpm;
                break;
            }

            let power = packet.power / 1000.0;
            let torque = packet.torque;
            let rpm = packet.current_engine_rpm;

            if power > data.peak {
                data.peak = power;
            }

            if torque > data.peak {
                data.peak = torque;
            }

            if let Some(&(last_rpm, _)) = data.power.last() {
                if rpm < last_rpm {
                    continue;
                }
            }

            data.power.push((rpm, power));
            data.torque.push((rpm, torque));
        }
    }

    Ok(data)
}
