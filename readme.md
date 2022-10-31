## 特点：
* 支持qemu和ch32vf407双平台，即可以在软件仿真环境中使用，也可以跑在真实的硬件开发板上。
* 采用cfg条件编译实现不同平台、不同特性的切换


## 如何构建：
进入到对应的app所在的crate中，执行`cargo xtask make`即可。一定要进入到app对应的crate目录中，否则cargo在运行的时候无法加载对应`.config文件夹下的配置信息`

TODO: 这些配置项应该由xtask在调用cargo时以命令行参数传过去。

## 如何切换平台
进入`apps`目录下感兴趣的APP对应的crate，修改这个crate的目录下`.config/config.toml`文件中的`"--cfg", "libos_port_ch32v307"`参数，目前可选的有`libos_port_ch32v307`和`libos_port_qemu_virt`。

另外，为了保证Rust-Analyzer插件可以正常工作，在`.vscode/settings.json`中需要调整对应的平台参数。


## 如何在CH32VF407开发板上进行调试
本项目使用VSCode进行开发，但是需要沁恒定制版本的openocd，所以需要首先安装沁恒的开发工具`MounRiver`。然后需要riscv版本的gdb调试器，可以从`https://github.com/xpack-dev-tools/riscv-none-embed-gcc-xpack`下载，我用的是`10.2.0-1.2`这个版本。另外需要给VSCode安装`cortex-debug`插件（虽然我们调的是riscv的处理器，但是cortex-debug这个插件还是挺好用的，我们通过配置文件把cortex-debug用作riscv的debug）。

上述工具安装完成以后，编辑项目根目录`./vscode/launch.json`的内容，将`gdbPath`、`serverpath`、`configFiles`几个配置项对应的路径修改为自己工具链的安装路径，如果需要在调试界面观察各个外设寄存器的值，需要把`svdFile`参数也设置好，可以在`MounRiver`的安装路径中找到`CH32V307xx.svd`这个文件。然后按F5就可以调试了。