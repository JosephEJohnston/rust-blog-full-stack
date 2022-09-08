-- MySQL dump 10.13  Distrib 8.0.30, for Win64 (x86_64)
--
-- Host: 127.0.0.1    Database: rust_blog
-- ------------------------------------------------------
-- Server version	8.0.30

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `article`
--

DROP TABLE IF EXISTS `article`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `article` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `user_id` bigint unsigned NOT NULL,
  `title` varchar(256) NOT NULL,
  `outline` varchar(1024) NOT NULL,
  `status` tinyint NOT NULL DEFAULT '0',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `modify_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=22 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `article`
--

LOCK TABLES `article` WRITE;
/*!40000 ALTER TABLE `article` DISABLE KEYS */;
INSERT INTO `article` VALUES (1,1,'测试文章插入','测试文章插入',0,'2022-08-19 12:42:12','2022-08-19 12:42:12'),(2,1,'测试文章插入3','测试文章插入3',0,'2022-09-01 10:22:55','2022-09-01 10:22:55'),(3,1,'测试文章插入4','测试文章插入4',0,'2022-09-01 10:28:28','2022-09-01 10:28:28'),(4,13,'点照真','ex',0,'2022-09-01 12:10:00','2022-09-01 12:10:00'),(5,1,'测试新建标题','测试新建描述',0,'2022-09-02 03:42:25','2022-09-02 03:42:25'),(6,1,'','',0,'2022-09-05 03:02:05','2022-09-05 03:02:05'),(7,1,'测试标题','测试描述',0,'2022-09-05 04:09:52','2022-09-05 04:09:52'),(8,1,'测试标题','',0,'2022-09-05 08:28:55','2022-09-05 08:28:55'),(9,1,'测试标题','测试主要描述',0,'2022-09-05 08:29:00','2022-09-05 08:29:00'),(10,1,'测试时区','测试时区',0,'2022-09-05 08:31:33','2022-09-05 08:31:33'),(11,1,'test redirect','test redirect',0,'2022-09-05 09:57:32','2022-09-05 09:57:32'),(12,1,'test redirect2','test redirect2',0,'2022-09-05 09:57:56','2022-09-05 09:57:56'),(13,1,'test3','test3',0,'2022-09-05 10:07:53','2022-09-05 10:07:53'),(14,1,'test5','test5',0,'2022-09-05 10:24:33','2022-09-05 10:24:33'),(15,1,'test6','test6',0,'2022-09-05 11:07:44','2022-09-05 11:07:44'),(16,1,'test7','test7',0,'2022-09-05 11:13:56','2022-09-05 11:13:56'),(17,1,'','',0,'2022-09-06 03:20:32','2022-09-06 03:20:32'),(18,1,'测试标题0906','测试描述 0906',0,'2022-09-06 07:39:35','2022-09-06 07:39:35'),(19,1,'测试 markdown','测试 markdown',0,'2022-09-08 11:10:41','2022-09-08 11:10:41'),(20,1,'测试 markdown 图片','测试 markdown 图片',0,'2022-09-08 12:13:37','2022-09-08 12:13:37'),(21,1,'测试 markdown 图片2','测试 markdown 图片2',0,'2022-09-08 12:16:16','2022-09-08 12:16:16');
/*!40000 ALTER TABLE `article` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2022-09-08 22:01:38
