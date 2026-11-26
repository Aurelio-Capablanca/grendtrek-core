# GrendTrekk : A Database schema/data migrator (mover) for any (RDBMS, excluding DB2 for LUW - i) to any (RDBMS, excluding DB2 for LUW - i) database

### Brief:

GrendTrekk as this title states it's a database migrator tool desgined for high throughput scenarios and, for critical ones, while also having in mind the efforts and consideration for custom schema and data (move) migrations, being that GrendTrekk allows to the user to pass to it the data types of the destination, the same as providing enough scripting for the sckeptical/conservative DBA to handle the migration on it's own, but not relying on the tool for the execution.

[considerations for the present as December 2026, and a note of the author]:

as I've worked extensively on this project more as a personal milestone, than actually on the pursue of knowledge rather than actually a way to seek profit in such a project that small and with a limited target of audiences that the intention of the present is to be a tool for Developers, DBA's, and overall users interested in a database migration of vendor to vendor, as here the term migration is used on behalf of chaning the very vendor of a database (ex. SQL Server) to a different one (ex. PostgreSQL), treating the data and it's structure as the very core of the database, limiting it to just it's structure and the data contained in it, as the project takes on a limit, hence until December 2026, it's not considered under the scope of the project to add a feature to migrate a database programmability (in any case this if considered under a scope of migration should be on the very programmer/executor of the tool, or if not, outsource to a LLM to rewrite it).

so in shorter words, the scope and limitations are set, being that the tool goal is to cover database migrations between SQL Server, PostgreSQL, SQLite, MySQL and Oracle, being that for the alpha version of the project it's only covered SQL Server to PostgreSQL,
