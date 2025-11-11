CREATE TABLE
    public.users (
        id uuid NOT NULL,
        username character varying(32) NOT NULL,
        password character varying(255) NOT NULL
    );

ALTER TABLE public.users
ADD CONSTRAINT users_pkey PRIMARY KEY (id);

ALTER TABLE public.users
ADD CONSTRAINT users_username_unique UNIQUE (username);