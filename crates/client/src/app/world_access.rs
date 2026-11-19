use shared::protocol::ServerConnection;

pub trait WorldAccess {
    fn list_worlds(&self) -> anyhow::Result<Vec<String>>;
    fn create_world(&self, name: &str, seed: u32) -> anyhow::Result<()>;
    fn enter_world(&self, name: &str) -> anyhow::Result<ServerConnection>;
    fn exit_world()
}
