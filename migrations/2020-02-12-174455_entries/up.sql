-- Your SQL goes here
CREATE TABLE entries (
   id			   BIGINT NOT NULL AUTO_INCREMENT,
   title		   VARCHAR(50) NULL DEFAULT 'Daily Entry',
   question		   VARCHAR(50) NOT NULL,
   answer		   TEXT NOT NULL,
   morning		   BOOL NOT NULL,
   entry_date	   DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
   PRIMARY KEY (id)
);
