学习 task-parser skill。

使用task-parser优化本文档，与用户进行沟通引导用户对任务进行进一步扩展和完善后，把优化后的清晰准确的任务要求保存到task-optimized.md，然后再基于task-optimized分析和创建提案。

非常重要！！！：你在了解了当前的项目情况对本文档内容进行理解和思考后，必须先根据 task-parser skill的要求对本文档进行分析并编写task-optimized.md！然后经过用户检阅并答复确认后才能开始创建提案。

完成下述需求：

我希望aide init生成的config.toml不再是自文档形式，而是一个正常的config.toml，但是我希望同时再生成一个config.md，专门用于配置讲解。

还有，实现aide config reset和aide config update，reset用于恢复配置文件默认值，update用于更新配置项，如果aide程序的版本高于当前项目的配置数据所记录的aide版本，可用aide config update，进行对配置项的更新补全，如果在新版本中弃用了某项旧配置，则将其注释。