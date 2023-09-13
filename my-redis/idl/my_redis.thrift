namespace rs my_redis

enum ItemType {
    Get,
    Set,
    Del,
    Ping,
    Subscribe,
    Publish,
}

struct Item {
    1: optional string key,
    2: optional string value,
    3: optional i32 expire_time,
    4: required ItemType request_type,
}

enum ResponseType {
    Success,
    Error,
}

struct ItemResponse {
    1: required ResponseType response_type,
    2: optional string value,
}

service ItemService  {
    ItemResponse RedisCommand (1: Item req, 2: bool is_from_master),
}
