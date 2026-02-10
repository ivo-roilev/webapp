-- MySQL Users Database Schema
-- Defines the users table for user account management

-- Create database if it doesn't exist
CREATE DATABASE IF NOT EXISTS webapp_db;
USE webapp_db;

-- Create users table with all required fields
CREATE TABLE IF NOT EXISTS users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(16) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    email VARCHAR(255),
    title VARCHAR(255),
    hobby VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Create unique index on username for efficient lookups
CREATE UNIQUE INDEX idx_username ON users(username);

-- Create index on created_at for time-based queries
CREATE INDEX idx_created_at ON users(created_at);

COMMIT;
