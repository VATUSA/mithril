CREATE TABLE `v3_api_key` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `code` VARCHAR(255) NOT NULL,
    `testing` BOOLEAN NOT NULL DEFAULT FALSE,
    `facility` VARCHAR(255) NULL,
    `notes` TEXT NULL,
    `created_at` BIGINT NOT NULL,
    `updated_at` BIGINT NULL,
    `deleted_at` BIGINT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `v3_api_key_code_unique` (`code`),
    KEY `v3_api_key_facility_index` (`facility`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `v3_webhook` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `facility` VARCHAR(255) NULL,
    `url` VARCHAR(2048) NOT NULL,
    `secret` VARCHAR(255) NOT NULL,
    `notes` TEXT NULL,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP NULL DEFAULT NULL,
    `deleted_at` TIMESTAMP NULL DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `v3_webhook_facility_index` (`facility`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
