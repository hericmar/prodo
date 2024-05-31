use uuid::Uuid;

pub struct Task {
    uid: Uuid,
    summary: String,
    description: String,
    author: u32,
}

pub struct CreateTask {
    summary: String,
    author: u32
}
