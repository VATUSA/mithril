/*M!999999\- enable the sandbox mode */ 
-- MariaDB dump 10.19-12.3.2-MariaDB, for Linux (x86_64)
--
-- Host: ngws-do-user-9538942-0.c.db.ondigitalocean.com    Database: cobalt
-- ------------------------------------------------------
-- Server version	8.0.45

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*M!100616 SET @OLD_NOTE_VERBOSITY=@@NOTE_VERBOSITY, NOTE_VERBOSITY=0 */;

--
-- Table structure for table `acl_actor_role`
--

DROP TABLE IF EXISTS `acl_actor_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `acl_actor_role` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `actor_id` int NOT NULL,
  `facility` varchar(4) NOT NULL,
  `role` varchar(120) NOT NULL,
  `grantor_cid` int NOT NULL,
  `granted_at` bigint NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `acl_user_role`
--

DROP TABLE IF EXISTS `acl_user_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `acl_user_role` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `cid` int NOT NULL,
  `facility` varchar(4) NOT NULL,
  `role` varchar(120) NOT NULL,
  `grantor_cid` int NOT NULL,
  `granted_at` bigint NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=823 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `actor`
--

DROP TABLE IF EXISTS `actor`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `actor` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(120) NOT NULL,
  `actor_type` varchar(40) NOT NULL,
  `facility_owner` varchar(4) DEFAULT NULL,
  `cid_owner` int DEFAULT NULL,
  `rate_limit_override` int NOT NULL DEFAULT '0',
  `rate_limit_bypass` tinyint(1) NOT NULL DEFAULT '0',
  `is_active` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` bigint NOT NULL,
  `created_by_cid` int NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `actor_token`
--

DROP TABLE IF EXISTS `actor_token`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `actor_token` (
  `id` int NOT NULL AUTO_INCREMENT,
  `actor_id` int NOT NULL,
  `comment` varchar(240) DEFAULT NULL,
  `token` varchar(120) NOT NULL,
  `is_active` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` bigint NOT NULL,
  `created_by_cid` int NOT NULL,
  `updated_at` bigint NOT NULL,
  `updated_by_cid` int NOT NULL,
  PRIMARY KEY (`id`),
  KEY `actor_id` (`actor_id`),
  CONSTRAINT `actor_token_ibfk_1` FOREIGN KEY (`actor_id`) REFERENCES `actor` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `event`
--

DROP TABLE IF EXISTS `event`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `event` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `title` varchar(240) NOT NULL,
  `body` text NOT NULL,
  `banner_image_url` text NOT NULL,
  `facility` varchar(4) NOT NULL,
  `start_time` bigint NOT NULL,
  `end_time` bigint NOT NULL,
  `created_at` bigint NOT NULL,
  `created_by` int NOT NULL,
  `updated_at` bigint NOT NULL,
  `updated_by` int NOT NULL,
  `review_status` varchar(40) DEFAULT NULL,
  `reviewed_by` int DEFAULT NULL,
  `reviewed_on` bigint DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_event_review_status_start_time` (`review_status`,`start_time`)
) ENGINE=InnoDB AUTO_INCREMENT=48 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `news_post`
--

DROP TABLE IF EXISTS `news_post`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `news_post` (
  `id` int NOT NULL AUTO_INCREMENT,
  `title` varchar(120) NOT NULL,
  `body` text NOT NULL,
  `author_cid` int NOT NULL,
  `post_time` bigint NOT NULL,
  `edit_time` bigint NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=65 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `schema_migrations`
--

DROP TABLE IF EXISTS `schema_migrations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `schema_migrations` (
  `version` bigint NOT NULL,
  `dirty` tinyint(1) NOT NULL,
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `transfer_history`
--

DROP TABLE IF EXISTS `transfer_history`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `transfer_history` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `cid` bigint NOT NULL,
  `from_facility` varchar(12) NOT NULL,
  `to_facility` varchar(12) NOT NULL,
  `reason` mediumtext NOT NULL,
  `requested_at` timestamp NOT NULL,
  `completed_at` timestamp NOT NULL,
  `approver_cid` bigint DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `transfer_request`
--

DROP TABLE IF EXISTS `transfer_request`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `transfer_request` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `cid` bigint NOT NULL,
  `from_facility` varchar(12) NOT NULL,
  `to_facility` varchar(12) NOT NULL,
  `reason` mediumtext NOT NULL,
  `created_at` timestamp NOT NULL,
  `status` varchar(40) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `user`
--

DROP TABLE IF EXISTS `user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `user` (
  `cid` bigint NOT NULL,
  `display_name` varchar(240) NOT NULL DEFAULT '',
  `controller_rating` int NOT NULL DEFAULT '0',
  `instructor_rating` int NOT NULL DEFAULT '0',
  `facility` varchar(4) NOT NULL,
  `discord_id` varchar(255) NOT NULL DEFAULT '',
  `last_promotion_time` timestamp NULL DEFAULT NULL,
  `last_transfer_time` timestamp NULL DEFAULT NULL,
  `last_competency_date` timestamp NULL DEFAULT NULL,
  `last_visit_time` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`cid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `user_rating_hours`
--

DROP TABLE IF EXISTS `user_rating_hours`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `user_rating_hours` (
  `cid` bigint NOT NULL,
  `rating` int NOT NULL,
  `hours` int NOT NULL,
  `last_check_time` timestamp NOT NULL,
  PRIMARY KEY (`cid`,`rating`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `user_visit`
--

DROP TABLE IF EXISTS `user_visit`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `user_visit` (
  `cid` bigint NOT NULL,
  `facility` varchar(4) NOT NULL,
  PRIMARY KEY (`cid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `v3_api_key`
--

DROP TABLE IF EXISTS `v3_api_key`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `v3_api_key` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `code` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `testing` tinyint(1) NOT NULL DEFAULT '0',
  `facility` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `notes` text COLLATE utf8mb4_unicode_ci,
  `created_at` bigint NOT NULL,
  `updated_at` bigint DEFAULT NULL,
  `deleted_at` bigint DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `v3_api_key_code_unique` (`code`),
  KEY `v3_api_key_facility_index` (`facility`)
) ENGINE=InnoDB AUTO_INCREMENT=45 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `v3_webhook`
--

DROP TABLE IF EXISTS `v3_webhook`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `v3_webhook` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `facility` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `url` varchar(2048) COLLATE utf8mb4_unicode_ci NOT NULL,
  `secret` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `notes` text COLLATE utf8mb4_unicode_ci,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NULL DEFAULT NULL,
  `deleted_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `v3_webhook_facility_index` (`facility`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `vatsim_user`
--

DROP TABLE IF EXISTS `vatsim_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `vatsim_user` (
  `cid` bigint NOT NULL,
  `name_first` varchar(240) NOT NULL,
  `name_last` varchar(240) NOT NULL,
  `email` varchar(240) NOT NULL,
  `rating` int NOT NULL,
  `pilotrating` int NOT NULL,
  `militaryrating` int NOT NULL,
  `suspend_date` datetime DEFAULT NULL,
  `registration_date` datetime DEFAULT NULL,
  `region_id` varchar(12) NOT NULL,
  `division_id` varchar(12) NOT NULL,
  `subdivision_id` varchar(12) DEFAULT NULL,
  `latest_rating_change` datetime DEFAULT NULL,
  `last_sync` datetime DEFAULT NULL,
  PRIMARY KEY (`cid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `visit_history`
--

DROP TABLE IF EXISTS `visit_history`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `visit_history` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `cid` bigint NOT NULL,
  `facility` varchar(12) NOT NULL,
  `reason` mediumtext NOT NULL,
  `requested_at` timestamp NOT NULL,
  `approved_at` timestamp NOT NULL,
  `approver_cid` bigint DEFAULT NULL,
  `removed_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `visit_request`
--

DROP TABLE IF EXISTS `visit_request`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `visit_request` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `cid` bigint NOT NULL,
  `facility` varchar(12) NOT NULL,
  `reason` mediumtext NOT NULL,
  `created_at` timestamp NOT NULL,
  `is_rejected` tinyint(1) DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*M!100616 SET NOTE_VERBOSITY=@OLD_NOTE_VERBOSITY */;

-- Dump completed on 2026-07-27 16:01:14
