DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD firstName ON TABLE user TYPE string;
DEFINE FIELD lastName ON TABLE user TYPE string;
DEFINE FIELD email ON TABLE user TYPE string
  ASSERT string::is::email($value);
DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;
DEFINE FIELD password ON TABLE user TYPE string