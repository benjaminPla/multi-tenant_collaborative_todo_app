DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'plaza_app') THEN
        CREATE ROLE plaza_app LOGIN NOSUPERUSER NOBYPASSRLS PASSWORD 'plaza_app';
    END IF;
END
$$;
