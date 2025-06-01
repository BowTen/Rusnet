use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
	NewRoom{
		password: String
	},
	JoinRoom{
		password: String
	}
}