DEFINE TABLE sys_user SCHEMAFULL;
DEFINE FIELD created_at ON TABLE sys_user TYPE datetime
DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE sys_user TYPE datetime
    DEFAULT time::now()
    VALUE time::now();
DEFINE FIELD username ON TABLE sys_user TYPE string;
DEFINE FIELD email ON TABLE sys_user TYPE string
  VALUE string::lowercase($value)
  ASSERT string::is::email($value);
DEFINE INDEX userEmailIndex ON TABLE sys_user COLUMNS email UNIQUE;
DEFINE FIELD password ON TABLE sys_user TYPE string;
DEFINE FIELD created_by ON TABLE sys_user TYPE option<record<sys_user>>;
DEFINE FIELD updated_by ON TABLE sys_user TYPE option<record<sys_user>>;
