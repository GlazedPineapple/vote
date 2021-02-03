CREATE TABLE "polls" (
	"id"	        TEXT NOT NULL UNIQUE,
	"title"	        TEXT NOT NULL,
	"moderators"	TEXT NOT NULL,
	"choices"	    TEXT NOT NULL,
	"timestamp"		TEXT NOT NULL,
	PRIMARY KEY("id")
)