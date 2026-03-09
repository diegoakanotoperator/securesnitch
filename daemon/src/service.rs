use tonic::{Request, Response, Status, Streaming};
use crate::rules_engine::protocol::ui_server::Ui;
use crate::rules_engine::protocol::{
    PingRequest, PingReply, Connection, Rule, Alert, Notification, 
    NotificationReply, ClientConfig, MsgResponse
};
use tokio_stream::wrappers::ReceiverStream;

#[derive(Default)]
pub struct SecureSnitchService {}

#[tonic::async_trait]
impl Ui for SecureSnitchService {
    async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingReply>, Status> {
        Ok(Response::new(PingReply {
            id: 0,
        }))
    }

    async fn ask_rule(&self, request: Request<Connection>) -> Result<Response<Rule>, Status> {
        let conn = request.into_inner();
        println!("Received AskRule request for process: {}", conn.process_path);
        
        Ok(Response::new(Rule {
            action: "deny".to_string(),
            duration: "until_restart".to_string(),
            enabled: true,
            ..Default::default()
        }))
    }

    async fn subscribe(&self, request: Request<ClientConfig>) -> Result<Response<ClientConfig>, Status> {
        Ok(Response::new(request.into_inner()))
    }

    type NotificationsStream = ReceiverStream<Result<Notification, Status>>;

    async fn notifications(&self, _request: Request<Streaming<NotificationReply>>) -> Result<Response<Self::NotificationsStream>, Status> {
        let (_tx, rx) = tokio::sync::mpsc::channel(1);
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn post_alert(&self, _request: Request<Alert>) -> Result<Response<MsgResponse>, Status> {
        Ok(Response::new(MsgResponse {
            id: 0,
        }))
    }
}
