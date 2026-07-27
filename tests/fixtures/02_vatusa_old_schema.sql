/*M!999999\- enable the sandbox mode */ 
-- MariaDB dump 10.19-12.3.2-MariaDB, for Linux (x86_64)
--
-- Host: ngws-do-user-9538942-0.c.db.ondigitalocean.com    Database: vatusa-old
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
-- Table structure for table `academy_basic_exam_emails`
--

DROP TABLE IF EXISTS `academy_basic_exam_emails`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `academy_basic_exam_emails` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `attempt_id` int NOT NULL,
  `student_id` int NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6916 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `academy_competency`
--

DROP TABLE IF EXISTS `academy_competency`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `academy_competency` (
  `id` int NOT NULL AUTO_INCREMENT,
  `cid` int DEFAULT NULL,
  `academy_course_id` int DEFAULT NULL,
  `completion_timestamp` timestamp NULL DEFAULT NULL,
  `expiration_timestamp` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `rating` int NOT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_academy_competency_cid` (`cid`)
) ENGINE=InnoDB AUTO_INCREMENT=2383 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `academy_course`
--

DROP TABLE IF EXISTS `academy_course`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `academy_course` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(120) NOT NULL,
  `list_order` int DEFAULT NULL,
  `moodle_enrol_id` int DEFAULT NULL,
  `moodle_quiz_id` int DEFAULT NULL,
  `passing_percent` int NOT NULL,
  `rating` int NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `academy_course_enrollment`
--

DROP TABLE IF EXISTS `academy_course_enrollment`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `academy_course_enrollment` (
  `id` int NOT NULL AUTO_INCREMENT,
  `cid` int NOT NULL,
  `academy_course_id` int NOT NULL,
  `assignment_timestamp` timestamp NULL DEFAULT NULL,
  `passed_timestamp` timestamp NULL DEFAULT NULL,
  `status` int NOT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=17221 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `academy_exam_assignments`
--

DROP TABLE IF EXISTS `academy_exam_assignments`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `academy_exam_assignments` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `student_id` int NOT NULL,
  `instructor_id` int NOT NULL,
  `moodle_uid` int NOT NULL,
  `course_id` int NOT NULL,
  `course_name` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `quiz_id` int NOT NULL,
  `rating_id` int NOT NULL,
  `attempt_emails_sent` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2484 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `action_log`
--

DROP TABLE IF EXISTS `action_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `action_log` (
  `id` int NOT NULL AUTO_INCREMENT,
  `from` int NOT NULL,
  `to` int NOT NULL,
  `log` text NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=897025 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `api_log`
--

DROP TABLE IF EXISTS `api_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `api_log` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `facility` varchar(3) NOT NULL,
  `datetime` datetime NOT NULL,
  `method` varchar(7) NOT NULL,
  `url` varchar(255) NOT NULL,
  `data` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `facility` (`facility`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `checklist_data`
--

DROP TABLE IF EXISTS `checklist_data`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `checklist_data` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `checklist_id` int unsigned NOT NULL,
  `item` varchar(255) NOT NULL,
  `order` int unsigned NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=30 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `checklists`
--

DROP TABLE IF EXISTS `checklists`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `checklists` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `active` int NOT NULL,
  `order` int unsigned NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `controller_eligibility_cache`
--

DROP TABLE IF EXISTS `controller_eligibility_cache`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `controller_eligibility_cache` (
  `cid` int NOT NULL,
  `competency_rating` int DEFAULT NULL,
  `competency_date` date DEFAULT NULL,
  `is_initial_selection` tinyint(1) DEFAULT NULL,
  `first_selection_date` date DEFAULT NULL,
  `has_consolidation_hours` tinyint(1) DEFAULT NULL,
  `consolidation_hours` float DEFAULT NULL,
  `last_promotion_date` date DEFAULT NULL,
  `last_transfer_date` date DEFAULT NULL,
  `last_visit_date` date DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`cid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `controller_training`
--

DROP TABLE IF EXISTS `controller_training`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `controller_training` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `student_cid` int unsigned NOT NULL,
  `instructor_cid` int unsigned NOT NULL,
  `facility` varchar(3) NOT NULL,
  `position` varchar(10) NOT NULL,
  `type` enum('Classroom','Live','Simulation','OTS Live','OTS Simulation') NOT NULL,
  `checklist_name` varchar(255) NOT NULL,
  `checklist_data` mediumtext NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `controllers`
--

DROP TABLE IF EXISTS `controllers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `controllers` (
  `cid` int unsigned NOT NULL,
  `fname` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `lname` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `email` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `facility` varchar(4) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `rating` int NOT NULL,
  `created_at` datetime DEFAULT NULL,
  `updated_at` datetime NOT NULL,
  `flag_needbasic` int NOT NULL DEFAULT '0',
  `flag_xferOverride` int NOT NULL DEFAULT '0',
  `facility_join` datetime NOT NULL,
  `flag_homecontroller` int NOT NULL,
  `remember_token` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `cert_update` int NOT NULL DEFAULT '0',
  `lastactivity` datetime NOT NULL,
  `flag_broadcastOptedIn` tinyint(1) NOT NULL DEFAULT '0',
  `flag_preventStaffAssign` tinyint(1) NOT NULL DEFAULT '0',
  `access_token` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `refresh_token` mediumtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `token_expires` bigint unsigned DEFAULT NULL,
  `discord_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `prefname` tinyint(1) NOT NULL,
  `prefname_date` datetime DEFAULT NULL,
  `last_cert_sync` datetime DEFAULT NULL,
  `flag_nameprivacy` tinyint(1) NOT NULL DEFAULT '0',
  `last_competency_date` date DEFAULT NULL,
  PRIMARY KEY (`cid`),
  UNIQUE KEY `cid_2` (`cid`),
  UNIQUE KEY `controllers_discord_id_unique` (`discord_id`),
  KEY `rating` (`rating`),
  KEY `fname` (`fname`),
  KEY `lname` (`lname`),
  KEY `cid` (`cid`),
  KEY `facility` (`facility`),
  KEY `controllers_homecontroller` (`flag_homecontroller`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `email_accounts`
--

DROP TABLE IF EXISTS `email_accounts`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `email_accounts` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `facility` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `username` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `cid` int NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=34 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `email_config`
--

DROP TABLE IF EXISTS `email_config`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `email_config` (
  `address` varchar(50) NOT NULL,
  `config` text,
  `destination` text,
  `modified_by` int DEFAULT NULL,
  `updated_at` text,
  PRIMARY KEY (`address`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `email_outbound`
--

DROP TABLE IF EXISTS `email_outbound`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `email_outbound` (
  `id` int NOT NULL AUTO_INCREMENT,
  `from_email` varchar(240) DEFAULT NULL,
  `from_name` varchar(240) DEFAULT NULL,
  `reply_to_email` varchar(240) DEFAULT NULL,
  `to_emails` text,
  `bcc_emails` text,
  `subject` varchar(400) NOT NULL,
  `body` text NOT NULL,
  `lock_key` varchar(40) DEFAULT NULL,
  `processed` tinyint(1) DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `email_templates`
--

DROP TABLE IF EXISTS `email_templates`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `email_templates` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `facility_id` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `template` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `body` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=80 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `exam_assignments`
--

DROP TABLE IF EXISTS `exam_assignments`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `exam_assignments` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `cid` int unsigned NOT NULL,
  `exam_id` int unsigned NOT NULL,
  `instructor_id` int unsigned NOT NULL,
  `assigned_date` datetime NOT NULL,
  `expire_date` datetime NOT NULL,
  PRIMARY KEY (`id`),
  KEY `cid` (`cid`,`exam_id`)
) ENGINE=InnoDB AUTO_INCREMENT=27911 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `exam_generated`
--

DROP TABLE IF EXISTS `exam_generated`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `exam_generated` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `cid` int unsigned NOT NULL,
  `exam_id` int unsigned NOT NULL,
  `question_id` int unsigned NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `exam_questions`
--

DROP TABLE IF EXISTS `exam_questions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `exam_questions` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `exam_id` int unsigned NOT NULL,
  `question` text NOT NULL,
  `type` int NOT NULL COMMENT '0 - multiple choice, 1 - true/false',
  `answer` varchar(255) NOT NULL,
  `alt1` varchar(255) NOT NULL,
  `alt2` varchar(255) NOT NULL,
  `alt3` varchar(255) NOT NULL,
  `notes` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=10520 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `exam_reassignments`
--

DROP TABLE IF EXISTS `exam_reassignments`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `exam_reassignments` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `cid` int unsigned NOT NULL,
  `exam_id` int unsigned NOT NULL,
  `reassign_date` datetime NOT NULL,
  `instructor_id` int unsigned NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5941 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `exam_results`
--

DROP TABLE IF EXISTS `exam_results`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `exam_results` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `exam_id` bigint unsigned NOT NULL,
  `exam_name` varchar(255) NOT NULL,
  `cid` int NOT NULL,
  `score` int NOT NULL,
  `passed` int NOT NULL,
  `date` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=68633 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `exam_results_data`
--

DROP TABLE IF EXISTS `exam_results_data`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `exam_results_data` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `result_id` bigint unsigned NOT NULL,
  `question` varchar(255) NOT NULL,
  `correct` varchar(255) NOT NULL,
  `selected` varchar(255) DEFAULT NULL,
  `notes` varchar(255) NOT NULL,
  `is_correct` int NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=593277 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `exams`
--

DROP TABLE IF EXISTS `exams`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `exams` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `facility_id` varchar(4) NOT NULL,
  `name` varchar(255) NOT NULL,
  `number` int NOT NULL,
  `is_active` int NOT NULL DEFAULT '1' COMMENT '0 - no, 1 - yes',
  `cbt_required` bigint unsigned DEFAULT '0',
  `retake_period` int NOT NULL DEFAULT '7' COMMENT 'number of days',
  `passing_score` int NOT NULL DEFAULT '70',
  `answer_visibility` enum('none','user_only','all','all_passed') NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=633 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `facilities`
--

DROP TABLE IF EXISTS `facilities`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `facilities` (
  `id` char(3) NOT NULL,
  `name` varchar(255) NOT NULL,
  `url` varchar(255) NOT NULL,
  `hosted_email_domain` varchar(255) DEFAULT NULL,
  `region` int NOT NULL,
  `atm` int unsigned NOT NULL,
  `datm` int unsigned NOT NULL,
  `ta` int unsigned NOT NULL,
  `ec` int unsigned NOT NULL,
  `fe` int unsigned NOT NULL,
  `wm` int unsigned NOT NULL,
  `uls_return` varchar(255) NOT NULL,
  `uls_devreturn` varchar(255) NOT NULL,
  `uls_secret` varchar(255) NOT NULL,
  `uls_jwk` text,
  `active` int NOT NULL DEFAULT '1',
  `apikey` varchar(25) NOT NULL,
  `ip` varchar(128) NOT NULL,
  `api_sandbox_key` varchar(255) NOT NULL,
  `api_sandbox_ip` varchar(128) NOT NULL,
  `apiv2_jwk` text,
  `welcome_text` mediumtext NOT NULL,
  `ace` int NOT NULL,
  `apiv2_jwk_dev` text,
  `uls_jwk_dev` varchar(255) NOT NULL,
  `url_dev` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COMMENT='Facility (ARTCC/CF/FIR) Listing';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `facility_trends`
--

DROP TABLE IF EXISTS `facility_trends`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `facility_trends` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `date` date NOT NULL,
  `facility` varchar(4) NOT NULL,
  `obs` int NOT NULL,
  `obsg30` int NOT NULL,
  `s1` int NOT NULL,
  `s2` int NOT NULL,
  `s3` int NOT NULL,
  `c1` int NOT NULL,
  `c3` int NOT NULL,
  `i1` int NOT NULL COMMENT 'I1 or greater',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `failed_jobs`
--

DROP TABLE IF EXISTS `failed_jobs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `failed_jobs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `uuid` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `connection` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `queue` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `payload` longtext CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `exception` longtext CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `failed_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `failed_jobs_uuid_unique` (`uuid`)
) ENGINE=InnoDB AUTO_INCREMENT=2019 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `flights`
--

DROP TABLE IF EXISTS `flights`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `flights` (
  `callsign` char(10) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `lat` varchar(15) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `long` varchar(15) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `hdg` int NOT NULL,
  `dest` varchar(4) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `dep` varchar(4) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `type` varchar(8) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  PRIMARY KEY (`callsign`),
  UNIQUE KEY `callsign` (`callsign`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `jobs`
--

DROP TABLE IF EXISTS `jobs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `jobs` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `type` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `data` mediumtext CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `status` enum('pending','running','success','failed') CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `knowledgebase_categories`
--

DROP TABLE IF EXISTS `knowledgebase_categories`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `knowledgebase_categories` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(256) NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `knowledgebase_questions`
--

DROP TABLE IF EXISTS `knowledgebase_questions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `knowledgebase_questions` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `category_id` int unsigned NOT NULL,
  `order` int unsigned NOT NULL,
  `question` text NOT NULL,
  `answer` mediumtext NOT NULL,
  `updated_by` int NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=33 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `login_tokens`
--

DROP TABLE IF EXISTS `login_tokens`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `login_tokens` (
  `token` varchar(255) NOT NULL,
  `cid` int unsigned NOT NULL,
  `timestamp` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `ip` varchar(128) NOT NULL,
  PRIMARY KEY (`token`),
  UNIQUE KEY `token` (`token`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `memberships`
--

DROP TABLE IF EXISTS `memberships`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `memberships` (
  `cid` int unsigned NOT NULL,
  `rating` int unsigned NOT NULL,
  `facility_id` varchar(3) NOT NULL,
  `type` int NOT NULL COMMENT '1 - Home, 2- Visitor',
  `joined` datetime NOT NULL,
  PRIMARY KEY (`cid`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COMMENT='Defines roster of facilities';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `migrations`
--

DROP TABLE IF EXISTS `migrations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `migrations` (
  `id` int NOT NULL,
  `migration` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `batch` int NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `oauth_clients`
--

DROP TABLE IF EXISTS `oauth_clients`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `oauth_clients` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(191) NOT NULL,
  `client_id` varchar(128) DEFAULT NULL,
  `client_secret` varchar(255) DEFAULT NULL,
  `redirect_uris` text,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  PRIMARY KEY (`id`,`name`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `oauth_logins`
--

DROP TABLE IF EXISTS `oauth_logins`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `oauth_logins` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `token` varchar(128) DEFAULT NULL,
  `code` longtext,
  `user_agent` varchar(255) DEFAULT NULL,
  `redirect_uri` varchar(255) DEFAULT NULL,
  `client_id` bigint unsigned DEFAULT NULL,
  `state` longtext,
  `code_challenge` longtext,
  `code_challenge_method` longtext,
  `scope` longtext,
  `c_id` bigint unsigned DEFAULT NULL,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `fk_oauth_logins_client` (`client_id`) -- ,
  -- CONSTRAINT `fk_oauth_logins_client` FOREIGN KEY (`client_id`) REFERENCES `oauth_clients` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `ots_evals`
--

DROP TABLE IF EXISTS `ots_evals`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `ots_evals` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `training_record_id` int DEFAULT NULL,
  `student_id` int NOT NULL,
  `instructor_id` int NOT NULL,
  `facility_id` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `exam_position` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `form_id` int unsigned NOT NULL,
  `notes` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci,
  `exam_date` date NOT NULL,
  `result` tinyint(1) NOT NULL,
  `signature` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `ots_evals_form_id_foreign` (`form_id`),
  CONSTRAINT `ots_evals_form_id_foreign` FOREIGN KEY (`form_id`) REFERENCES `ots_evals_forms` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=2048 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `ots_evals_forms`
--

DROP TABLE IF EXISTS `ots_evals_forms`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `ots_evals_forms` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `rating_id` int NOT NULL,
  `position` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `instructor_notes` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci,
  `is_statement` tinyint(1) NOT NULL DEFAULT '0',
  `description` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `active` tinyint(1) NOT NULL DEFAULT '1',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `ots_evals_forms_name_unique` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `ots_evals_indicator_results`
--

DROP TABLE IF EXISTS `ots_evals_indicator_results`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `ots_evals_indicator_results` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `perf_indicator_id` int unsigned NOT NULL,
  `eval_id` int unsigned NOT NULL,
  `result` smallint NOT NULL,
  `comment` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `ots_evals_indicator_results_perf_indicator_id_foreign` (`perf_indicator_id`),
  KEY `ots_evals_indicator_results_eval_id_foreign` (`eval_id`),
  CONSTRAINT `ots_evals_indicator_results_eval_id_foreign` FOREIGN KEY (`eval_id`) REFERENCES `ots_evals` (`id`) ON DELETE CASCADE,
  CONSTRAINT `ots_evals_indicator_results_perf_indicator_id_foreign` FOREIGN KEY (`perf_indicator_id`) REFERENCES `ots_evals_perf_indicators` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=71135 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `ots_evals_perf_cats`
--

DROP TABLE IF EXISTS `ots_evals_perf_cats`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `ots_evals_perf_cats` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `label` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `form_id` int unsigned NOT NULL,
  `order` int NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `ots_evals_perf_cats_form_id_foreign` (`form_id`),
  CONSTRAINT `ots_evals_perf_cats_form_id_foreign` FOREIGN KEY (`form_id`) REFERENCES `ots_evals_forms` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=21 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `ots_evals_perf_indicators`
--

DROP TABLE IF EXISTS `ots_evals_perf_indicators`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `ots_evals_perf_indicators` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `perf_cat_id` int unsigned NOT NULL,
  `label` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `help_text` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci,
  `header_type` smallint NOT NULL DEFAULT '0',
  `is_commendable` tinyint(1) DEFAULT NULL,
  `is_required` tinyint(1) DEFAULT NULL,
  `can_unsat` tinyint(1) DEFAULT NULL,
  `order` int NOT NULL,
  `extra_options` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `ots_evals_perf_indicators_perf_cat_id_foreign` (`perf_cat_id`),
  CONSTRAINT `ots_evals_perf_indicators_perf_cat_id_foreign` FOREIGN KEY (`perf_cat_id`) REFERENCES `ots_evals_perf_cats` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=283 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `password_resets`
--

DROP TABLE IF EXISTS `password_resets`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `password_resets` (
  `email` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `token` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`token`),
  KEY `password_resets_email_index` (`email`),
  KEY `password_resets_token_index` (`token`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `policies`
--

DROP TABLE IF EXISTS `policies`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `policies` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `ident` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `category` int unsigned NOT NULL,
  `title` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `slug` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `description` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `extension` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `effective_date` date DEFAULT NULL,
  `perms` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `visible` tinyint(1) NOT NULL,
  `order` smallint unsigned NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `policies_category_foreign` (`category`),
  CONSTRAINT `policies_category_foreign` FOREIGN KEY (`category`) REFERENCES `policy_categories` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=50 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `policy_categories`
--

DROP TABLE IF EXISTS `policy_categories`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `policy_categories` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `order` smallint unsigned NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `promotions`
--

DROP TABLE IF EXISTS `promotions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `promotions` (
  `id` int NOT NULL AUTO_INCREMENT,
  `cid` int NOT NULL,
  `grantor` int unsigned NOT NULL,
  `to` int NOT NULL,
  `from` int NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  `exam` date NOT NULL,
  `examiner` int unsigned NOT NULL,
  `position` varchar(11) NOT NULL,
  `eval_id` int unsigned DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_promotions_cid` (`cid`)
) ENGINE=InnoDB AUTO_INCREMENT=20579 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `promotionstest`
--

DROP TABLE IF EXISTS `promotionstest`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `promotionstest` (
  `id` int NOT NULL AUTO_INCREMENT,
  `cid` int NOT NULL,
  `grantor` int unsigned NOT NULL,
  `to` int NOT NULL,
  `from` int NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  `exam` date NOT NULL,
  `examiner` int unsigned NOT NULL,
  `position` varchar(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=13044 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `push_log`
--

DROP TABLE IF EXISTS `push_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `push_log` (
  `id` int NOT NULL AUTO_INCREMENT,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `updated_at` date DEFAULT NULL,
  `title` text NOT NULL,
  `message` text NOT NULL,
  `submitted_by` text NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `ratings`
--

DROP TABLE IF EXISTS `ratings`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `ratings` (
  `id` int NOT NULL,
  `short` varchar(3) NOT NULL,
  `long` varchar(18) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `return_paths`
--

DROP TABLE IF EXISTS `return_paths`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `return_paths` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `order` int NOT NULL,
  `facility_id` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `url` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=265 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `role_titles`
--

DROP TABLE IF EXISTS `role_titles`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `role_titles` (
  `role` varchar(12) NOT NULL,
  `title` varchar(128) NOT NULL,
  PRIMARY KEY (`role`),
  UNIQUE KEY `role` (`role`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `roles`
--

DROP TABLE IF EXISTS `roles`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `roles` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `cid` int unsigned NOT NULL,
  `facility` varchar(3) NOT NULL,
  `role` varchar(12) NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `roles_role` (`role`)
) ENGINE=InnoDB AUTO_INCREMENT=6712618 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `sessions`
--

DROP TABLE IF EXISTS `sessions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `sessions` (
  `id` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `user_id` int unsigned DEFAULT NULL,
  `ip_address` varchar(45) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci DEFAULT NULL,
  `user_agent` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci,
  `payload` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `last_activity` int NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `sessions_id_unique` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `solo_certs`
--

DROP TABLE IF EXISTS `solo_certs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `solo_certs` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `cid` int unsigned NOT NULL,
  `position` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `expires` date NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00',
  `updated_at` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2677 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `stats_archive`
--

DROP TABLE IF EXISTS `stats_archive`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `stats_archive` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `date` date NOT NULL,
  `data` mediumtext NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=67 DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `survey_assignments`
--

DROP TABLE IF EXISTS `survey_assignments`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `survey_assignments` (
  `id` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `survey_id` int NOT NULL,
  `facility` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `rating` int NOT NULL,
  `misc_data` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `completed` int NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `survey_questions`
--

DROP TABLE IF EXISTS `survey_questions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `survey_questions` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `survey_id` int NOT NULL,
  `question` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `data` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `order` int NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `survey_submissions`
--

DROP TABLE IF EXISTS `survey_submissions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `survey_submissions` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `survey_id` int NOT NULL,
  `question_id` int NOT NULL,
  `response` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `facility` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `rating` int NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `surveys`
--

DROP TABLE IF EXISTS `surveys`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `surveys` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `facility` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `name` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `tickets`
--

DROP TABLE IF EXISTS `tickets`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `tickets` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `cid` int NOT NULL,
  `subject` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `body` mediumtext CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `status` enum('Open','Closed') CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `facility` char(3) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `assigned_to` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `notes` mediumtext CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `priority` enum('Low','Normal','High') CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `created_at` datetime NOT NULL DEFAULT '0000-00-00 00:00:00',
  `updated_at` datetime NOT NULL DEFAULT '0000-00-00 00:00:00',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5803 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `tickets_history`
--

DROP TABLE IF EXISTS `tickets_history`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `tickets_history` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `ticket_id` bigint unsigned NOT NULL,
  `entry` mediumtext NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`),
  KEY `ticket_id` (`ticket_id`)
) ENGINE=InnoDB AUTO_INCREMENT=35930 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `tickets_notes`
--

DROP TABLE IF EXISTS `tickets_notes`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `tickets_notes` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `ticket_id` int NOT NULL,
  `cid` int NOT NULL,
  `note` mediumtext CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00',
  `updated_at` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `tickets_replies`
--

DROP TABLE IF EXISTS `tickets_replies`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `tickets_replies` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `ticket_id` int NOT NULL,
  `cid` int NOT NULL,
  `body` mediumtext CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00',
  `updated_at` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=12159 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `tmu_colors`
--

DROP TABLE IF EXISTS `tmu_colors`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `tmu_colors` (
  `id` varchar(4) NOT NULL,
  `black` text,
  `brown` text COMMENT '1',
  `blue` text COMMENT '2',
  `gray` text COMMENT '3',
  `green` text COMMENT '4',
  `lime` text COMMENT '5',
  `cyan` text COMMENT '6',
  `orange` text COMMENT '7',
  `red` text COMMENT '9',
  `purple` text COMMENT '10',
  `white` text COMMENT '11',
  `yellow` text COMMENT '12',
  `violet` text COMMENT '13',
  `guide` text,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `tmu_facilities`
--

DROP TABLE IF EXISTS `tmu_facilities`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `tmu_facilities` (
  `id` varchar(4) NOT NULL,
  `parent` varchar(4) DEFAULT NULL,
  `name` varchar(255) NOT NULL,
  `coords` text NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `tmu_notices`
--

DROP TABLE IF EXISTS `tmu_notices`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `tmu_notices` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `tmu_facility_id` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `priority` smallint NOT NULL,
  `message` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `start_date` datetime NOT NULL,
  `expire_date` datetime DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `is_delay` tinyint(1) NOT NULL,
  `is_pref_route` tinyint(1) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=221 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `training_blocks`
--

DROP TABLE IF EXISTS `training_blocks`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `training_blocks` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `facility` varchar(3) NOT NULL COMMENT 'Facility ID, ZAE = VATUSA Academy',
  `order` int NOT NULL,
  `name` varchar(255) NOT NULL,
  `level` enum('Senior Staff','Staff','I1','C1','S1','ALL') NOT NULL DEFAULT 'ALL',
  `visible` tinyint(1) NOT NULL DEFAULT '1',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=301 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `training_chapters`
--

DROP TABLE IF EXISTS `training_chapters`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `training_chapters` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `blockid` bigint unsigned NOT NULL,
  `order` int NOT NULL,
  `name` varchar(255) NOT NULL,
  `url` varchar(255) NOT NULL,
  `visible` tinyint(1) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `blockid` (`blockid`),
  CONSTRAINT `training_chapters_ibfk_1` FOREIGN KEY (`blockid`) REFERENCES `training_blocks` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=560 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `training_progress`
--

DROP TABLE IF EXISTS `training_progress`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `training_progress` (
  `cid` int unsigned NOT NULL,
  `chapterid` bigint unsigned NOT NULL,
  `date` datetime NOT NULL,
  KEY `chapterid` (`chapterid`),
  CONSTRAINT `training_progress_ibfk_1` FOREIGN KEY (`chapterid`) REFERENCES `training_chapters` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `training_records`
--

DROP TABLE IF EXISTS `training_records`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `training_records` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `student_id` int NOT NULL,
  `instructor_id` int NOT NULL,
  `session_date` datetime NOT NULL,
  `facility_id` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `position` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `duration` time NOT NULL,
  `movements` int DEFAULT NULL,
  `score` int DEFAULT NULL,
  `notes` text CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `location` smallint NOT NULL,
  `ots_status` smallint NOT NULL,
  `ots_eval_id` int DEFAULT NULL,
  `is_cbt` tinyint(1) NOT NULL,
  `solo_granted` tinyint(1) NOT NULL,
  `modified_by` int DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=54433 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `transfers`
--

DROP TABLE IF EXISTS `transfers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `transfers` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `cid` int unsigned NOT NULL,
  `to` varchar(3) NOT NULL,
  `from` varchar(3) NOT NULL,
  `reason` text NOT NULL,
  `status` int NOT NULL COMMENT '0-pending,1-accepted,2-rejected',
  `actiontext` text NOT NULL,
  `actionby` int unsigned NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`),
  KEY `cid` (`cid`,`created_at`),
  KEY `created_at` (`created_at`)
) ENGINE=InnoDB AUTO_INCREMENT=823517 DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `uls_tokens`
--

DROP TABLE IF EXISTS `uls_tokens`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `uls_tokens` (
  `facility` varchar(4) NOT NULL,
  `token` varchar(128) NOT NULL,
  `date` datetime NOT NULL,
  `ip` varchar(128) NOT NULL,
  `cid` int unsigned NOT NULL,
  `expired` int NOT NULL DEFAULT '0',
  PRIMARY KEY (`token`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `users` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `email` varchar(255) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `password` varchar(60) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `remember_token` varchar(100) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci DEFAULT NULL,
  `created_at` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00',
  `updated_at` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00',
  PRIMARY KEY (`id`),
  UNIQUE KEY `users_email_unique` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `visits`
--

DROP TABLE IF EXISTS `visits`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `visits` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `cid` int unsigned NOT NULL,
  `facility` varchar(3) CHARACTER SET utf8mb3 COLLATE utf8mb3_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4829 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*M!100616 SET NOTE_VERBOSITY=@OLD_NOTE_VERBOSITY */;

-- Dump completed on 2026-07-08 16:53:15
