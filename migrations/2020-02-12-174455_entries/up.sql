-- Your SQL goes here
CREATE TABLE entries (
   id INT NOT NULL AUTO_INCREMENT,
   title VARCHAR(50) NULL DEFAULT 'Daily Entry',
   question ENUM(
	'grateful_for', 'make_today_great',
	'affirmation', 'amazing_thing', 'improve_today') NOT NULL,
   answer ENUM('one', 'two', 'three') NOT NULL,
   entry_date DATETIME DEFAULT CURRENT_TIMESTAMP,
   PRIMARY KEY (id)
);
