rust项目的文件查找

    1. 优先查找 xxx.rs 文件 i. main.rs 、 lib.rs 、 mod.rs 中的 mod xxx; 默认优先查找同级目 录下的 xxx.rs 文件； 
    ii. 其他文件 yyy.rs 中的 mod xxx; 默认优先查找同级目录的 yyy 目录下 的 xxx.rs 文件； 
    2. 如果 xxx.rs 不存在，则查找 xxx/mod.rs 文件，即 xxx 目录下的 mod.rs 文件。