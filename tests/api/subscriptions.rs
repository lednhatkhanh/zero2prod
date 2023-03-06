use sqlx::PgPool;

use crate::helpers::spawn_app;

#[sqlx::test]
async fn subscribe_returns_a_200_for_valid_form_data(pool: PgPool) -> sqlx::Result<()> {
    // Arrange
    let app = spawn_app(pool.clone()).await;
    let mut connection = pool.acquire().await?;

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = app.post_subscriptions(body.to_string()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");

    Ok(())
}

#[sqlx::test]
async fn subscribe_returns_a_400_when_data_is_missing(pool: PgPool) -> sqlx::Result<()> {
    // Arrange
    let app = spawn_app(pool).await;

    // Act
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.to_string()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }

    Ok(())
}

#[sqlx::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_empty(
    pool: PgPool,
) -> sqlx::Result<()> {
    // Arrange
    let app = spawn_app(pool).await;

    // Act
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.to_string()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 OK when the payload was {}.",
            error_message
        );
    }

    Ok(())
}
