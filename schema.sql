-- CreateTable
CREATE TABLE IF NOT EXISTS errors (
   id uuid DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
   app_id varchar(16) NOT NULL,
   user_id varchar(16),
   message varchar(255) NOT NULL,
   location varchar(255) NOT NULL,
   context varchar,
   trace varchar NOT NULL,
   created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
