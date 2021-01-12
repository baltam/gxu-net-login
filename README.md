一个用于自动登录广西大学校园网的程序，使用 Rust 语言编写，可跨不同平台使用。

## 使用方法

1. 在 [Releases](https://github.com/baltam/gxu-net-login/releases) 页面中下载最新版本的压缩包，并解压；
2. 使用 VS Code 或记事本等文本编辑器修改 `config.ini`，根据注释中的提示，填入您的用户名和密码；
3. 双击 `gxu-net-login.exe` 即可登录。（运行 `gxu-net-login.bat` 可以看到日志信息。）

## 设置开机启动

将 `gxu-net-login.exe` 设为开机启动（以 Windows 10 为例）：

1. 右击 `gxu-net-login.exe`，选择「复制」；
2. 打开资源管理器，点击地址栏空白位置，输入 `shell:startup`，回车；
3. 在打开的文件夹中右击，选择「粘贴快捷方式」。

## 故障排除

运行 `gxu-net-login.bat` 可以看到程序日志。如果程序没有正常工作，请先检查日志信息。

如果程序不断提示登录失败，请首先检查 `config.ini` 中的用户名和密码是否有错误。用初始用户名和密码是无法登录的。

如果在日志窗口中看到如下信息：

```
thread 'main' panicked at '...'
```

说明网关接口可能有变化，请联系开发者更新程序。

## 联系作者

如果您认为程序中有 bug，请直接提出 issue。
