use tracing::{instrument, warn};
use tracing::log::debug;
use common_define::event::lora_gateway::GatewayStatus;
use common_define::lorawan_bridge::{GatewayEventType, GatewayUpData, GatewayUpDataHeader, RXPK};
use crate::{man::lora::LoRaGateManager, DeviceResult};
use crate::event::gateway::GatewayEvent;
use crate::man::lora::LoRaGate;
use crate::service::lorawan_node::{node_data, PushData};

pub(crate) fn gateway_event(
    event: GatewayUpData,
)  {
    tokio::spawn(async move {
        gateway_event_process_warp(event).await;
    });
}

#[instrument]
async fn gateway_event_process_warp(
    event: GatewayUpData,
) {
    if let Err(e) = gateway_event_process(event).await {
        warn!("{}", e);
    }
}

async fn gateway_event_process(
    event: GatewayUpData,
) -> DeviceResult {
    let mut gw = LoRaGateManager::get_gate(event.eui).await?;
    gw.update_version(event.version).await?;
    GatewayEvent::gateway_state(gw.id, event.clone()).await?;
    
    let (header, event) = event.into_inner();
    match event {
        GatewayEventType::Status(status) => gateway_status(status, gw).await?,
        GatewayEventType::PushData(pks) => gateway_push_data(pks, gw, header).await?,
        GatewayEventType::Pull => gateway_pull_data(header, gw).await?,
        GatewayEventType::TxAck => gateway_txack_data(header, gw).await?,
    };
    Ok(())
}

async fn gateway_status(status: GatewayStatus, gw: LoRaGate) -> DeviceResult  {
    debug!("gateway status");
    Ok(())
}

async fn gateway_push_data(pks: Vec<RXPK>, mut gw: LoRaGate, header: GatewayUpDataHeader) -> DeviceResult  {
    for pk in pks {
        let rssi = pk.rssi;
        let data = PushData {
            gateway: gw.id,
            eui: header.eui,
            token: header.token,
            version: header.version,
            time: header.time,
            pk,
        };
        tokio::spawn(node_data(gw.clone(), rssi, data));
    } 
    gw.push_ack(header.token, header.source.ip).await?;
    Ok(())
}

async fn gateway_pull_data(header: GatewayUpDataHeader, mut gw: LoRaGate) -> DeviceResult {
    debug!("pull data");
    gw.update_down(header.source.ip).await?;
    gw.pull_ack(header.token).await?;
    Ok(())
}

async fn gateway_txack_data(header: GatewayUpDataHeader, gw: LoRaGate) -> DeviceResult {
    debug!("txack");
    Ok(())
}