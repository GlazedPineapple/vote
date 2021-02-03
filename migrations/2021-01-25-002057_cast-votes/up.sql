CREATE TABLE "cast_votes" (
	"id"	    TEXT NOT NULL UNIQUE,
	"user"	    TEXT NOT NULL,
	"poll"	    TEXT NOT NULL,
	"ranking"	TEXT NOT NULL,
	"timestamp"		TEXT NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("poll") REFERENCES "polls"("id")
)