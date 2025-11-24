use uuid::Uuid;

#[allow(dead_code)]
pub fn parse_uuid(s: &str) -> Result<Uuid, uuid::Error> {
    Uuid::parse_str(s)
}
