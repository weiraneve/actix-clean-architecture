DROP TABLE IF EXISTS `hero`, `team`, `log`;

CREATE TABLE `hero` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL COMMENT '英雄名',
  `line` int NOT NULL COMMENT '英雄分路',
  `is_pick` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否被选择',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ;

INSERT INTO `hero` (`id`, `name`, `line`, `is_pick`)
VALUES
	(1,'盘古','1',0),
	(2,'雅典娜','1',0),
	(3,'梦奇','1',0),
	(4,'蒙恬','1',0),
	(5,'曹操','1',0),
	(6,'哪吒','1',0),
	(7,'杨戬','1',0),
	(8,'达摩','1',0),
	(9,'八戒','1',0),
	(10,'白起','1',0),
	(11,'司空震','1',0),
	(13,'苏烈','1',0),
	(14,'杨玉环','2',0),
	(15,'嫦娥','2',0),
	(16,'沈梦溪','2',0),
	(17,'西施','2',0),
	(18,'弈星','2',0),
	(19,'女娲','2',0),
	(20,'周瑜','2',0),
	(21,'司马懿','2',0),
	(22,'扁鹊','2',0),
	(23,'海月','2',0),
	(24,'高渐离','2',0),
	(25,'成吉思汗','3',0),
	(26,'戈娅','3',0),
	(27,'太乙真人','4',0),
	(28,'鲁班大师','4',0),
	(29,'刘邦','4',0),
	(30,'盾山','4',0),
	(31,'牛魔','4',0),
	(32,'桑启','4',0),
	(33,'鬼谷','4',0),
	(34,'云中君','5',0),
	(35,'裴擒虎','5',0),
	(36,'暃','5',0),
	(37,'百里玄策','5',0),
	(38,'露娜','5',0),
	(39,'镜','5',0),
	(40,'阿古朵','5',0),
	(41,'刘备','5',0),
	(42,'云缨','5',0);


CREATE TABLE `log` (
  `id` int NOT NULL AUTO_INCREMENT,
  `team_id` int NOT NULL COMMENT '队伍id',
  `pick_group` varchar(100) CHARACTER SET utf8mb4  NOT NULL COMMENT '抽取组合',
  `time` datetime NOT NULL COMMENT '记录时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ;

CREATE TABLE `team` (
  `id` int NOT NULL AUTO_INCREMENT,
  `encrypt_code` varchar(20) CHARACTER SET utf8mb4 DEFAULT NULL COMMENT '队伍秘钥',
  `pick_content` varchar(100) CHARACTER SET utf8mb4 DEFAULT NULL COMMENT '抽取结果',
  `is_picked` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否抽取过',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ;


INSERT INTO `team` (`id`, `encrypt_code`, `pick_content`, `is_picked`, `update_time`)
VALUES
	(1, 'apqmf1jk3n1mwe1', '', 0, '2022-11-18 00:00:00'),
	(2, 'dyeqndasdiko121', '', 0, '2022-11-18 00:00:00'),
	(3, 'sowuedasd99we1d', '', 0, '2022-11-18 00:00:00'),
	(4, 'fqnxjs2131j4j12', '', 0, '2022-11-18 00:00:00'),
	(5, 'gpdyeewa9das88d', '', 0, '2022-11-18 00:00:00'),
	(6, 'hdmxb19dsc967vm', '', 0, '2022-11-18 00:00:00'),
	(7, 'jdmejwfvusdn120', '', 0, '2022-11-18 00:00:00'),
	(8, 'kejwuq908fvd87v', '', 0, '2022-11-18 00:00:00'),
	(9, 'asd', '', 0, '2022-11-18 00:00:00');
