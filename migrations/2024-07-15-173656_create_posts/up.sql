CREATE TABLE "post" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    video_url VARCHAR(2048) NULL,
    image_url VARCHAR(2048) NULL,
    user_id INTEGER NOT NULL,   

    CONSTRAINT FK_post_user FOREIGN KEY(user_id) REFERENCES "user"(id)
);

CREATE INDEX NX_user_id ON "post"(user_id);
CREATE INDEX NX_created_at ON "post"(created_at)