--
-- PostgreSQL database dump
--

-- Dumped from database version 15.8
-- Dumped by pg_dump version 16.3

-- Started on 2024-09-21 22:18:48

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 215 (class 1259 OID 16394)
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO postgres;

--
-- TOC entry 214 (class 1259 OID 16387)
-- Name: categories; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.categories (
    url character varying NOT NULL,
    category_id integer NOT NULL,
    config character varying,
    category_name character varying NOT NULL
);


ALTER TABLE public.categories OWNER TO postgres;

--
-- TOC entry 217 (class 1259 OID 16407)
-- Name: item_trackers; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.item_trackers (
    id integer NOT NULL,
    name character varying NOT NULL,
    currency character varying,
    icon text,
    link character varying,
    favorite boolean,
    category_id integer NOT NULL
);


ALTER TABLE public.item_trackers OWNER TO postgres;

--
-- TOC entry 218 (class 1259 OID 24616)
-- Name: notifications; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.notifications (
    id integer NOT NULL,
    message character varying NOT NULL,
    icon character varying NOT NULL,
    color character varying NOT NULL,
    date timestamp with time zone NOT NULL
);


ALTER TABLE public.notifications OWNER TO postgres;

--
-- TOC entry 216 (class 1259 OID 16402)
-- Name: price_data; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.price_data (
    item_tracker integer NOT NULL,
    date timestamp with time zone NOT NULL,
    price double precision NOT NULL
);


ALTER TABLE public.price_data OWNER TO postgres;

--
-- TOC entry 3413 (class 0 OID 16394)
-- Dependencies: 215
-- Data for Name: _sqlx_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public._sqlx_migrations (version, description, installed_on, success, checksum, execution_time) FROM stdin;
\.


--
-- TOC entry 3412 (class 0 OID 16387)
-- Dependencies: 214
-- Data for Name: categories; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.categories (url, category_id, config, category_name) FROM stdin;
\.


--
-- TOC entry 3415 (class 0 OID 16407)
-- Dependencies: 217
-- Data for Name: item_trackers; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.item_trackers (id, name, currency, icon, link, favorite, category_id) FROM stdin;
\.


--
-- TOC entry 3416 (class 0 OID 24616)
-- Dependencies: 218
-- Data for Name: notifications; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.notifications (id, message, icon, color, date) FROM stdin;
\.


--
-- TOC entry 3414 (class 0 OID 16402)
-- Dependencies: 216
-- Data for Name: price_data; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.price_data (item_tracker, date, price) FROM stdin;
\.


--
-- TOC entry 3261 (class 2606 OID 16401)
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- TOC entry 3259 (class 2606 OID 16429)
-- Name: categories categories_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_pkey PRIMARY KEY (category_id);


--
-- TOC entry 3265 (class 2606 OID 16413)
-- Name: item_trackers item_trackers_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.item_trackers
    ADD CONSTRAINT item_trackers_pkey PRIMARY KEY (id);


--
-- TOC entry 3267 (class 2606 OID 24622)
-- Name: notifications notifications_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.notifications
    ADD CONSTRAINT notifications_pkey PRIMARY KEY (id);


--
-- TOC entry 3263 (class 2606 OID 24624)
-- Name: price_data price_data_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.price_data
    ADD CONSTRAINT price_data_pkey PRIMARY KEY (item_tracker, date, price);


--
-- TOC entry 3269 (class 2606 OID 16430)
-- Name: item_trackers category; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.item_trackers
    ADD CONSTRAINT category FOREIGN KEY (category_id) REFERENCES public.categories(category_id) NOT VALID;


--
-- TOC entry 3268 (class 2606 OID 16419)
-- Name: price_data item_tracker; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.price_data
    ADD CONSTRAINT item_tracker FOREIGN KEY (item_tracker) REFERENCES public.item_trackers(id) NOT VALID;


-- Completed on 2024-09-21 22:18:48

--
-- PostgreSQL database dump complete
--

