CREATE TABLE IF NOT EXISTS public.categories (
    url character varying NOT NULL,
    category_id integer NOT NULL,
    config character varying,
    category_name character varying NOT NULL,
    PRIMARY KEY (category_id)
);


CREATE TABLE IF NOT EXISTS public.item_trackers (
    id integer NOT NULL,
    name character varying NOT NULL,
    currency character varying,
    icon text,
    link character varying,
    favorite boolean,
    category_id integer NOT NULL,
    FOREIGN KEY (category_id) REFERENCES public.categories(category_id) ON DELETE CASCADE,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.notifications (
    id integer NOT NULL,
    message character varying NOT NULL,
    icon character varying NOT NULL,
    color character varying NOT NULL,
    date timestamp with time zone NOT NULL,
    PRIMARY KEY (id)
);


CREATE TABLE IF NOT EXISTS public.price_data (
    item_tracker integer NOT NULL,
    date timestamp with time zone NOT NULL,
    price double precision NOT NULL,
    FOREIGN KEY (item_tracker) REFERENCES public.item_trackers(id) ON DELETE CASCADE,
    PRIMARY KEY (item_tracker, date)
);
