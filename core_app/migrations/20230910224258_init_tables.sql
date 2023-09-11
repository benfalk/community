CREATE TABLE IF NOT EXISTS households
(
	id		INTEGER PRIMARY KEY NOT NULL,
	address		TEXT	            NOT NULL
);

CREATE TABLE IF NOT EXISTS household_members
(
	id		INTEGER PRIMARY KEY NOT NULL,
	household_id	INTEGER		    NOT NULL,
	first_name	TEXT	            NOT NULL,
	last_name	TEXT                NOT NULL,
	email		TEXT                UNIQUE COLLATE NOCASE,
	cell_number	TEXT,
	FOREIGN KEY(household_id) REFERENCES households(id) ON DELETE CASCADE
);
