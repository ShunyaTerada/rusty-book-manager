pub mod model;

use redis::{AsyncCommands, Client};
use shared::{config::RedisConfig, error::AppResult};

use self::model::{RedisKey, RedisValue};

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    //Redis接続用のクライアントを初期化する
    pub fn new(config: &RedisConfig) -> AppResult<Self> {
        let client = Client::open(format!("redis://{}:{}", config.host, config.port))?;
        Ok(Self { client })
    }

    pub async fn set_ex<T: RedisKey>(&self, key: &T, value: &T::Value, ttl: u64) -> AppResult<()> {
        //ttl（Time to Live）ネットワーク上でのデータの存命期間
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.set_ex::<_, _, ()>(key.inner(), value.inner(), ttl)
            .await?;
        Ok(())
    }

    //アクセストークンからユーザーIDを取得
    pub async fn get<T: RedisKey>(&self, key: &T) -> AppResult<Option<T::Value>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let result: Option<String> = conn.get(key.inner()).await?;
        result.map(T::Value::try_from).transpose()
    }

    //指定したキーに該当するデータを削除
    pub async fn delete<T: RedisKey>(&self, key: &T) -> AppResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.del::<_, ()>(key.inner()).await?;
        Ok(())
    }

    //接続確認（ヘルスチェック用）
    pub async fn try_connect(&self) -> AppResult<()> {
        self.client.get_multiplexed_async_connection().await?;
        Ok(())
    }
}
