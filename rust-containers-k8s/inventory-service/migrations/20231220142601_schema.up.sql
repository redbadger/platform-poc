-- Add up migration script here
--
-- PostgreSQL database dump
--
-- Dumped from database version 16.1 (Debian 16.1-1.pgdg120+1)
-- Dumped by pg_dump version 16.1 (Homebrew)
SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
-- SELECT pg_catalog.set_config('search_path', DEFAULT, false);
SET search_path TO DEFAULT;
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;
SET default_tablespace = '';
SET default_table_access_method = heap;
--
-- Name: t_inventory; Type: TABLE; Schema: public; Owner: postgres
--
CREATE TABLE public.t_inventory (
id bigint NOT NULL,
quantity integer,
sku_code character varying(255)
);
--
-- Name: t_inventory_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--
ALTER TABLE public.t_inventory ALTER COLUMN id ADD GENERATED BY DEFAULT AS IDENTITY (
SEQUENCE NAME public.t_inventory_id_seq START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1
);
--
-- Name: t_inventory t_inventory_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--
ALTER TABLE ONLY public.t_inventory
ADD CONSTRAINT t_inventory_pkey PRIMARY KEY (id);
--
-- PostgreSQL database dump complete
