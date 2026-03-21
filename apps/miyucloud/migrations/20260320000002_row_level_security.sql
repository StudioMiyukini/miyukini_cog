-- Miyukini Cloud: RLS helper function — ISO 27001 A.8.3
-- Split from policies to avoid sqlx statement-splitting issues with dollar-quoting.

CREATE OR REPLACE FUNCTION auth.current_app_user_id()
RETURNS uuid AS $FUNC$
BEGIN
    RETURN current_setting('app.current_user_id', true)::uuid;
EXCEPTION
    WHEN OTHERS THEN
        RETURN NULL;
END;
$FUNC$ LANGUAGE plpgsql STABLE;
