use std::env;

use anyhow::{Context, Result};
use infrastructure_rdb::Configuration;
use sqlx::{Postgres, Transaction};

#[tokio::main]
async fn main() -> Result<()> {
    let database_uri = env::var("DATABASE_URL")
        .context("データ投入先として環境変数 `DATABASE_URL` を定義してください")?;
    let pool = Configuration::new(1, database_uri).connect().await?;
    let mut transaction = pool.begin().await?;
    data(&mut transaction).await?;
    transaction.commit().await?;
    Ok(())
}

async fn data(transaction: &mut Transaction<'_, Postgres>) -> Result<()> {
    sqlx::query!(
        r#"
            -- ユーザーを挿入
            INSERT INTO users (id, name, email)
            VALUES
                ('user-01HBCCGK3MG5HA7GJG25BGV6PJ', 'aaa', 'aaa@example.com'),
                ('user-01HBCCGK3MH7XKWBDHXSWCAPWA', 'bbb', 'bbb@example.com'),
                ('user-01HBCCGK3MS53D8NM6EYZ0KZEH', 'ccc', 'ccc@example.com');
        "#
    )
    .execute(&mut **transaction)
    .await?;

    sqlx::query!(
        r#"
            -- ボードを挿入
            INSERT INTO boards (id, title)
            VALUES
                ('board-01HBCCGK3MH83RJ4Y8AVECQ5W9', 'yarukoto'),
                ('board-01HBCCGK3M3039H2QQEYD94TMS', 'wishlist'),
                ('board-01HBCCGK3M18C851FA0067MRPF', 'monster');
        "#
    )
    .execute(&mut **transaction)
    .await?;

    sqlx::query!(
        r#"
            -- カラムを挿入
            INSERT INTO columns (id, title)
            VALUES
                ('column-01HBCCGK3MAWDZKS74M1DEJQ54', 'TODO'),
                ('column-01HBCCGK3MR41MEZWGJERC5PHD', 'doing'),
                ('column-01HBCCGK3MDQRSF7X7EGKBMAY8', 'DONE'),
                ('column-01HBCCGK3MDD8M1T47N4MDB6AA', 'wish'),
                ('column-01HBCCGK3MMEFTBS3SJ73CK96K', 'bought'),
                ('column-01HBCCGK3M9BMDD7Z16JQNX3QC', 'pending'),
                ('column-01HBCCGK3MTAFEVEFAQMFE2W43', 'challenge'),
                ('column-01HBCCGK3M3SA44D9SCJVR5D8X', 'got');
        "#
    )
    .execute(&mut **transaction)
    .await?;

    sqlx::query!(
        r#"
            -- カードを挿入
            INSERT INTO cards (id, title, description, column_id)
            VALUES
                ('c0', '掃除', '', 'column-01HBCCGK3MAWDZKS74M1DEJQ54'),
                ('c1', '洗濯', '', 'column-01HBCCGK3MAWDZKS74M1DEJQ54'),
                ('c2', '食事', '', 'column-01HBCCGK3MAWDZKS74M1DEJQ54'),
                ('c3', 'ゴミ出し', '', 'column-01HBCCGK3MR41MEZWGJERC5PHD'),
                ('c4', 'ランタン', '', 'column-01HBCCGK3MDQRSF7X7EGKBMAY8'),
                ('c5', '本棚', 'いろいろ', 'column-01HBCCGK3MDQRSF7X7EGKBMAY8'),
                ('c6', '床下三兄弟', '', 'column-01HBCCGK3M9BMDD7Z16JQNX3QC'),
                ('c7', 'ダソッキー', '', 'column-01HBCCGK3M9BMDD7Z16JQNX3QC'),
                ('c8', 'ゴハッチュウ', '', 'column-01HBCCGK3M9BMDD7Z16JQNX3QC'),
                ('c9', 'センサーライト', '', 'column-01HBCCGK3MMEFTBS3SJ73CK96K'),
                ('c10', '飲み物', '', 'column-01HBCCGK3MMEFTBS3SJ73CK96K'),
                ('c11', '米', '', 'column-01HBCCGK3MMEFTBS3SJ73CK96K'),
                ('c12', '常備薬', '', 'column-01HBCCGK3MMEFTBS3SJ73CK96K'),
                ('c13', '石鹸', '', 'column-01HBCCGK3MMEFTBS3SJ73CK96K'),
                ('c14', '買い物', '', 'column-01HBCCGK3MDD8M1T47N4MDB6AA'),
                ('c15', '洗い物', '', 'column-01HBCCGK3MDD8M1T47N4MDB6AA');
        "#
    )
    .execute(&mut **transaction)
    .await?;

    sqlx::query!(
        r#"
            -- ユーザーとボードの関連付けを挿入
            INSERT INTO user_board_relations (user_id, board_id)
            VALUES
                ('user-01HBCCGK3MG5HA7GJG25BGV6PJ', 'board-01HBCCGK3MH83RJ4Y8AVECQ5W9'),
                ('user-01HBCCGK3MG5HA7GJG25BGV6PJ', 'board-01HBCCGK3M18C851FA0067MRPF'),
                ('user-01HBCCGK3MH7XKWBDHXSWCAPWA', 'board-01HBCCGK3M3039H2QQEYD94TMS');
        "#
    )
    .execute(&mut **transaction)
    .await?;

    sqlx::query!(
        r#"
            -- ボードとカラムの関連付けを挿入
            INSERT INTO board_column_relations (board_id, column_id)
            VALUES
                ('board-01HBCCGK3MH83RJ4Y8AVECQ5W9', 'column-01HBCCGK3MAWDZKS74M1DEJQ54'),
                ('board-01HBCCGK3MH83RJ4Y8AVECQ5W9', 'column-01HBCCGK3MR41MEZWGJERC5PHD'),
                ('board-01HBCCGK3MH83RJ4Y8AVECQ5W9', 'column-01HBCCGK3MDQRSF7X7EGKBMAY8'),
                ('board-01HBCCGK3M3039H2QQEYD94TMS', 'column-01HBCCGK3MDD8M1T47N4MDB6AA'),
                ('board-01HBCCGK3M3039H2QQEYD94TMS', 'column-01HBCCGK3MMEFTBS3SJ73CK96K'),
                ('board-01HBCCGK3M3039H2QQEYD94TMS', 'column-01HBCCGK3M9BMDD7Z16JQNX3QC'),
                ('board-01HBCCGK3M3039H2QQEYD94TMS', 'column-01HBCCGK3MTAFEVEFAQMFE2W43'),
                ('board-01HBCCGK3M3039H2QQEYD94TMS', 'column-01HBCCGK3M3SA44D9SCJVR5D8X');
        "#
    )
    .execute(&mut **transaction)
    .await?;

    Ok(())
}
