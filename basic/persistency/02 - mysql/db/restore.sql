CREATE DATABASE IF NOT EXISTS `clients_rust_db` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;
USE `clients_rust_db`;

CREATE TABLE IF NOT EXISTS `clients` (
  `id` int AUTO_INCREMENT PRIMARY KEY,
  `name` varchar(150) NOT NULL,
  `phone` varchar(20) NOT NULL
);

INSERT INTO `clients` (`id`, `name`, `phone`) VALUES
(1, 'John Doe', '11 9999-9991'),
(2, 'Jane Smith', '11 9999-9992'),
(3, 'Alice Johnson', '11 9999-9993'),
(4, 'Bob Brown', '11 9999-9994');
