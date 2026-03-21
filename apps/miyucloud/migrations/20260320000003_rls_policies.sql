-- Miyukini Cloud: RLS policies — ISO 27001 A.8.3

ALTER TABLE storage.folders ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS folders_user_isolation ON storage.folders;
CREATE POLICY folders_user_isolation ON storage.folders
    USING (user_id = auth.current_app_user_id() OR auth.current_app_user_id() IS NULL);

ALTER TABLE storage.files ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS files_user_isolation ON storage.files;
CREATE POLICY files_user_isolation ON storage.files
    USING (user_id = auth.current_app_user_id() OR auth.current_app_user_id() IS NULL);

ALTER TABLE storage.shares ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS shares_user_isolation ON storage.shares;
CREATE POLICY shares_user_isolation ON storage.shares
    USING (created_by = auth.current_app_user_id() OR auth.current_app_user_id() IS NULL);

ALTER TABLE auth.sessions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS sessions_user_isolation ON auth.sessions;
CREATE POLICY sessions_user_isolation ON auth.sessions
    USING (user_id = auth.current_app_user_id() OR auth.current_app_user_id() IS NULL);

ALTER TABLE caldav.calendars ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS calendars_user_isolation ON caldav.calendars;
CREATE POLICY calendars_user_isolation ON caldav.calendars
    USING (owner_id = auth.current_app_user_id() OR auth.current_app_user_id() IS NULL);

ALTER TABLE carddav.address_books ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS addressbooks_user_isolation ON carddav.address_books;
CREATE POLICY addressbooks_user_isolation ON carddav.address_books
    USING (owner_id = auth.current_app_user_id() OR auth.current_app_user_id() IS NULL);
