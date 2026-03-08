学习 task-parser skill。

使用task-parser优化本文档，与用户进行沟通引导用户对任务进行进一步扩展和完善后，把优化后的清晰准确的任务要求保存到task-optimized.md，然后再基于task-optimized分析和创建提案。

非常重要！！！：你在了解了当前的项目情况对本文档内容进行理解和思考后，必须先根据 task-parser skill的要求对本文档进行分析并编写task-optimized.md！然后经过用户检阅并答复确认后才能开始创建提案。

完成下述需求：

我希望aide init时同时在用户主目录下的.aide目录（不存在则自动创建）下创建一份config.toml，之后项目目录中的config.toml从这里复制，

如果用户主目录下已有全局配置文件，则aide init不需要再次创建或覆盖，直接复制它到项目目录（工作目录）即可。

添加--global支持，执行aide init --global时，不需要改动当前工作目录下的文件，也不用在当前工作目录创建.aide目录，只要在用户主目录下创建.aide/config.toml，当文件已存在时无动作，仅提示已存在。

执行aide config update --global时，同上，仅对用户主目录下的配置文件进行更新（差不多相当于在用户主目录下执行aide update），

