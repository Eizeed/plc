# Project line counter

## How to use:<br>
`plc -p [PATH] -e [FILE EXTENSION] [OPTIONS]`

## Default behavior
- hidden directories are ignored
- comments and documentation are ignored
- default extension is '.rs'
- default path is directory where it was called

## Path
`-p --path`
#### example
`plc -p /home/user/some_project`<br/>
check files in provided directory.<br/>

## File Extension
`-e --extension`
#### example
`plc -e .rs .py .lua`<br/>
check all files with given extensions.<br/>

## Options
- `-v --verbose` log the flow of app
- `-a --hidden` check files in hidden directories
- `-d --docs` count documentation
- `-c --comments` count comments
- `-f --fixme` count FIXME comments as another instance
- `-t --todo` count TODO comments as another instance

##### RUST SPECIFIC
- `-u --units` count structures, functions, impl blocks and declarative macros. If `-u` is provided with any other extension it will be ignored and set to false.
