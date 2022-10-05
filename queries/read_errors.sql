--! errors
SELECT
    created_at
FROM
    errors
WHERE
    app_id = :app_id;

--! errors_queried
SELECT
    id,
    user_id,
    message,
    location,
    context,
    trace,
    created_at
FROM
    errors
WHERE ( app_id = :app_id)
OR (
    message LIKE CONCAT(:query::text, '%%') OR
    location  LIKE CONCAT(:query::text, '%%')
)
ORDER BY created_at DESC
LIMIT :limit
OFFSET :offset;
