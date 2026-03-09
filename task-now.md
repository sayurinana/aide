学习 task-parser skill。

使用task-parser优化本文档，与用户进行沟通引导用户对任务进行进一步扩展和完善后，把优化后的清晰准确的任务要求保存到task-optimized.md，然后再基于task-optimized分析和创建提案。

非常重要！！！：你在了解了当前的项目情况对本文档内容进行理解和思考后，必须先根据 task-parser skill的要求对本文档进行分析并编写task-optimized.md！然后经过用户检阅并答复确认后才能开始创建提案。

完成下述需求：

在配置文件中添加几项配置：
- 下载缓存路径（默认为download-buffer）
- 完成安装后是否删除下载缓存文件（默认为是，删除）
- 工具程序安装路径（默认为utils）
- plantuml 程序包下载链接（默认值见下文）

> 使用的相对路径都是相对于用户目录下的.aide目录，例如utils就是~/.aide/utils

主要是为了我希望plantuml这种工具要能被直接集成到本程序中，不依靠外部环境支持。

我制作了一个plantuml的可执行程序文件打包，可以脱离java运行，上传到了github中，链接是 https://github.com/sayurinana/agent-aide/releases/download/resource-001/plantuml-1.2025.4-linux-x64.tar.gz ，我希望把这个作为下载使用的plantuml程序包默认链接。

我已经在当前项目目录的lib目录下下载好了所需的压缩包，并使用`tar zxf`对它完成了解压，把解压后的文件移动到了utils目录下，
这里的lib就差不多相对于实际运行时的`~/.aide`目录，你可以查看一下lib的目录结构。

我希望使用aide init --global时，检测是否配置好了plantuml可执行程序，plantuml是否可用，
例如按照我上面给出的默认配置，拼接得出plantuml的路径就是`~/.aide/utils/plantuml/bin/plantuml`，
如果不可用，则提示用户是否现在自动进行下载和解压等操作。

且运行aide -V时自动检测plantuml可用性，如不可用则提示进行下载安装，如可用则同时显示plantuml的版本。
