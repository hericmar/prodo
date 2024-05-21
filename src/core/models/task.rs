use uuid::Uuid;

pub struct Task {
    uid: Uuid,
    summary: String,
    description: String,
    author: Uuid,
}

pub struct CreateTask {
    summary: String
}
