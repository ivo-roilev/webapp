-- MySQL Users Database Schema
-- Defines the users table for user account management

-- Create dedicated application user with minimal privileges
CREATE USER IF NOT EXISTS 'webapp_user'@'%' IDENTIFIED BY 'webapp_dev_password';
GRANT SELECT, INSERT, UPDATE, DELETE ON webapp_db.* TO 'webapp_user'@'%';
FLUSH PRIVILEGES;

-- Create database if it doesn't exist
CREATE DATABASE IF NOT EXISTS webapp_db;
USE webapp_db;

-- Create users table (core authentication data)
CREATE TABLE IF NOT EXISTS users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(16) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Create user_profiles table (standard profile information)
CREATE TABLE IF NOT EXISTS user_profiles (
    user_id INT PRIMARY KEY,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    email VARCHAR(255),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create user_metadata table (flexible properties)
CREATE TABLE IF NOT EXISTS user_metadata (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    parent_property VARCHAR(255),
    property VARCHAR(255) NOT NULL,
    value TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create unique index on username for efficient lookups
SET @index_exists = (SELECT COUNT(*) FROM information_schema.statistics
                     WHERE table_schema = 'webapp_db' AND table_name = 'users' AND index_name = 'idx_username');
SET @sql = IF(@index_exists = 0, 'CREATE UNIQUE INDEX idx_username ON users(username)', 'SELECT ''Index idx_username already exists'' AS info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Create indexes on foreign keys for performance
SET @index_exists = (SELECT COUNT(*) FROM information_schema.statistics
                     WHERE table_schema = 'webapp_db' AND table_name = 'user_metadata' AND index_name = 'idx_metadata_user');
SET @sql = IF(@index_exists = 0, 'CREATE INDEX idx_metadata_user ON user_metadata(user_id)', 'SELECT ''Index idx_metadata_user already exists'' AS info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Create index on created_at for time-based queries
SET @index_exists = (SELECT COUNT(*) FROM information_schema.statistics
                     WHERE table_schema = 'webapp_db' AND table_name = 'users' AND index_name = 'idx_created_at');
SET @sql = IF(@index_exists = 0, 'CREATE INDEX idx_created_at ON users(created_at)', 'SELECT ''Index idx_created_at already exists'' AS info');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

COMMIT;
