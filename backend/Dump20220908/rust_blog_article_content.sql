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
-- Table structure for table `article_content`
--

DROP TABLE IF EXISTS `article_content`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `article_content` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `article_id` bigint unsigned NOT NULL,
  `content` mediumtext NOT NULL,
  `status` tinyint NOT NULL,
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `modify_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=21 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `article_content`
--

LOCK TABLES `article_content` WRITE;
/*!40000 ALTER TABLE `article_content` DISABLE KEYS */;
INSERT INTO `article_content` VALUES (1,1,'测试内容表',0,'2022-08-24 19:13:46','2022-08-24 19:13:46'),(2,1,'test article content insert',0,'2022-08-30 04:14:02','2022-08-30 04:14:02'),(3,4,'ea enim irure deserunt ipsum',0,'2022-09-01 12:10:00','2022-09-01 12:10:00'),(4,5,'测试新建内容',0,'2022-09-02 03:42:25','2022-09-02 03:42:25'),(5,6,'',0,'2022-09-05 03:02:05','2022-09-05 03:02:05'),(6,7,'测试内容',0,'2022-09-05 04:09:52','2022-09-05 04:09:52'),(7,8,'测试内容',0,'2022-09-05 08:28:55','2022-09-05 08:28:55'),(8,9,'测试内容',0,'2022-09-05 08:29:00','2022-09-05 08:29:00'),(9,10,'测试时区',0,'2022-09-05 08:31:33','2022-09-05 08:31:33'),(10,11,'test redirect',0,'2022-09-05 09:57:33','2022-09-05 09:57:33'),(11,12,'test redirect2',0,'2022-09-05 09:57:56','2022-09-05 09:57:56'),(12,13,'test3',0,'2022-09-05 10:07:53','2022-09-05 10:07:53'),(13,14,'test5',0,'2022-09-05 10:24:33','2022-09-05 10:24:33'),(14,15,'test6',0,'2022-09-05 11:07:44','2022-09-05 11:07:44'),(15,16,'test7',0,'2022-09-05 11:13:56','2022-09-05 11:13:56'),(16,17,'',0,'2022-09-06 03:20:32','2022-09-06 03:20:32'),(17,18,'测试内容',0,'2022-09-06 07:39:35','2022-09-06 07:39:35'),(18,19,'# Installation\n\n## Download tarball\n\nYou can download the latest release tarball directly from [releases][releases]\n\n## Bower\n\n    bower install showdown\n\n## npm (server-side)\n\n    npm install showdown\n\n## CDN\n\nYou can also use one of several CDNs available: \n\n* rawgit CDN\n\n        https://cdn.rawgit.com/showdownjs/showdown/<version tag>/dist/showdown.min.js\n\n* cdnjs\n\n        https://cdnjs.cloudflare.com/ajax/libs/showdown/<version tag>/showdown.min.js\n\n![Showdown][sd-logo]\n\n[sd-logo]: https://raw.githubusercontent.com/showdownjs/logo/master/dist/logo.readme.png\n[releases]: https://github.com/showdownjs/showdown/releases\n[atx]: http://www.aaronsw.com/2002/atx/intro\n[setext]: https://en.wikipedia.org/wiki/Setext\n\n---------\n',0,'2022-09-08 11:10:41','2022-09-08 11:10:41'),(19,20,'![](http://www.example.com/image.jpg)',0,'2022-09-08 12:13:37','2022-09-08 12:13:37'),(20,21,'![test_img](https://upload.wikimedia.org/wikipedia/commons/thumb/6/61/HTML5_logo_and_wordmark.svg/1200px-HTML5_logo_and_wordmark.svg.png)\n![test_img](/static/resource/img/logo-w.png)',0,'2022-09-08 12:16:16','2022-09-08 12:16:16');
/*!40000 ALTER TABLE `article_content` ENABLE KEYS */;
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
