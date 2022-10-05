--! insert_error
INSERT INTO errors (app_id, user_id, message, location, context, trace)
    VALUES (:app_id, :user_id, :message, :location, :context, :trace);
