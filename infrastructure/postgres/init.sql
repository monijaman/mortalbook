-- Initialize Mortalbook Database
CREATE SCHEMA IF NOT EXISTS public;

-- Users Table
CREATE TABLE IF NOT EXISTS users (
  id VARCHAR(36) PRIMARY KEY,
  email VARCHAR(255) UNIQUE NOT NULL,
  username VARCHAR(255) UNIQUE NOT NULL,
  full_name VARCHAR(255),
  hashed_password VARCHAR(255) NOT NULL,
  is_active BOOLEAN DEFAULT TRUE,
  is_admin BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP NULL
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_created_at ON users(created_at DESC);

-- Memorials Table
CREATE TABLE IF NOT EXISTS memorials (
  id VARCHAR(36) PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  date_of_birth DATE NOT NULL,
  date_of_death DATE NOT NULL,
  biography TEXT,
  created_by VARCHAR(36),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  status VARCHAR(50) DEFAULT 'active',
  deleted_at TIMESTAMP NULL
);

CREATE INDEX idx_memorials_status ON memorials(status);
CREATE INDEX idx_memorials_created_at ON memorials(created_at DESC);
CREATE INDEX idx_memorials_created_by ON memorials(created_by);

-- Memorial Media Table
CREATE TABLE IF NOT EXISTS memorial_media (
  id VARCHAR(36) PRIMARY KEY,
  memorial_id VARCHAR(36) NOT NULL,
  url VARCHAR(2048) NOT NULL,
  media_type VARCHAR(50),
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (memorial_id) REFERENCES memorials(id) ON DELETE CASCADE
);

CREATE INDEX idx_memorial_media_memorial_id ON memorial_media(memorial_id);

-- Comments Table
CREATE TABLE IF NOT EXISTS comments (
  id VARCHAR(36) PRIMARY KEY,
  memorial_id VARCHAR(36) NOT NULL,
  user_id VARCHAR(36),
  content TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP NULL,
  FOREIGN KEY (memorial_id) REFERENCES memorials(id) ON DELETE CASCADE,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX idx_comments_memorial_id ON comments(memorial_id);
CREATE INDEX idx_comments_user_id ON comments(user_id);
CREATE INDEX idx_comments_created_at ON comments(created_at DESC);

-- Admin Logs Table
CREATE TABLE IF NOT EXISTS admin_logs (
  id VARCHAR(36) PRIMARY KEY,
  user_id VARCHAR(36) NOT NULL,
  action VARCHAR(255) NOT NULL,
  resource_type VARCHAR(255),
  resource_id VARCHAR(36),
  description TEXT,
  ip_address VARCHAR(45),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_admin_logs_user_id ON admin_logs(user_id);
CREATE INDEX idx_admin_logs_created_at ON admin_logs(created_at DESC);
CREATE INDEX idx_admin_logs_action ON admin_logs(action);

-- Analytics Events Table
CREATE TABLE IF NOT EXISTS analytics_events (
  id VARCHAR(36) PRIMARY KEY,
  event_type VARCHAR(255) NOT NULL,
  user_id VARCHAR(36),
  data TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_analytics_events_event_type ON analytics_events(event_type);
CREATE INDEX idx_analytics_events_user_id ON analytics_events(user_id);
CREATE INDEX idx_analytics_events_created_at ON analytics_events(created_at DESC);

-- Create seed admin user (password: admin123)
INSERT INTO users (id, email, username, full_name, hashed_password, is_admin, is_active)
VALUES (
  'admin-user-001',
  'admin@mortalbook.local',
  'admin',
  'Administrator',
  '$2b$12$R9h7cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ee8DuY41sBfJfsDm',
  true,
  true
)
ON CONFLICT (email) DO NOTHING;
