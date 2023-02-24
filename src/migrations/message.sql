--
-- PostgreSQL database dump
--

-- Dumped from database version 15.2
-- Dumped by pg_dump version 15.2

-- Started on 2023-02-24 13:47:55

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
-- TOC entry 215 (class 1259 OID 16410)
-- Name: message; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.message (
    id smallint NOT NULL,
    message text NOT NULL,
    sent_at timestamp(6) with time zone NOT NULL,
    replying_to smallint,
    user_id smallint NOT NULL,
    active boolean DEFAULT true NOT NULL,
    room_id smallint NOT NULL
);


ALTER TABLE public.message OWNER TO postgres;

--
-- TOC entry 220 (class 1259 OID 16568)
-- Name: message_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public.message ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.message_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 216 (class 1259 OID 16423)
-- Name: room; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.room (
    id smallint NOT NULL,
    name character varying(50) NOT NULL,
    description text,
    admin smallint[] NOT NULL
);


ALTER TABLE public.room OWNER TO postgres;

--
-- TOC entry 221 (class 1259 OID 16569)
-- Name: room_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public.room ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.room_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 214 (class 1259 OID 16399)
-- Name: user; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."user" (
    id smallint NOT NULL,
    username character varying(50) NOT NULL,
    email character varying(255) NOT NULL,
    joined_at time(6) without time zone NOT NULL,
    active boolean DEFAULT true NOT NULL,
    password text NOT NULL
);


ALTER TABLE public."user" OWNER TO postgres;

--
-- TOC entry 218 (class 1259 OID 16566)
-- Name: user_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public."user" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.user_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 217 (class 1259 OID 16544)
-- Name: user_room; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_room (
    id smallint NOT NULL,
    user_id smallint NOT NULL,
    room_id smallint NOT NULL
);


ALTER TABLE public.user_room OWNER TO postgres;

--
-- TOC entry 219 (class 1259 OID 16567)
-- Name: user_room_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public.user_room ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.user_room_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 3197 (class 2606 OID 16417)
-- Name: message message_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.message
    ADD CONSTRAINT message_pkey PRIMARY KEY (id);


--
-- TOC entry 3199 (class 2606 OID 16555)
-- Name: room room_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.room
    ADD CONSTRAINT room_pkey PRIMARY KEY (id);


--
-- TOC entry 3191 (class 2606 OID 16573)
-- Name: user unique_email; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT unique_email UNIQUE (email);


--
-- TOC entry 3193 (class 2606 OID 16571)
-- Name: user unique_username; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT unique_username UNIQUE (username);


--
-- TOC entry 3195 (class 2606 OID 16404)
-- Name: user user_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);


--
-- TOC entry 3201 (class 2606 OID 16548)
-- Name: user_room user_room_id; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_room
    ADD CONSTRAINT user_room_id PRIMARY KEY (id);


--
-- TOC entry 3204 (class 2606 OID 16556)
-- Name: user_room FK_user_room_room; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_room
    ADD CONSTRAINT "FK_user_room_room" FOREIGN KEY (room_id) REFERENCES public.room(id) NOT VALID;


--
-- TOC entry 3205 (class 2606 OID 16549)
-- Name: user_room FK_user_room_user; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_room
    ADD CONSTRAINT "FK_user_room_user" FOREIGN KEY (user_id) REFERENCES public."user"(id) NOT VALID;


--
-- TOC entry 3202 (class 2606 OID 16574)
-- Name: message room_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.message
    ADD CONSTRAINT room_id FOREIGN KEY (room_id) REFERENCES public.room(id) NOT VALID;


--
-- TOC entry 3203 (class 2606 OID 16418)
-- Name: message user_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.message
    ADD CONSTRAINT user_id FOREIGN KEY (user_id) REFERENCES public."user"(id);


-- Completed on 2023-02-24 13:47:55

--
-- PostgreSQL database dump complete
--

