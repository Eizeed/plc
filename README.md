# Project line counter

### Roadmap:
1. Make better gitignore algo to use several gitignore files for certain folders
2. Make verbose flag to show all recursive directory visits
3. Make debug flag to show all outputs through programm

### How to use:<br>
    plc -p <PATH> -e <FILE EXTENSION>

## Default behavior
it will use directory where it was called with extension `.rs`

### features:
1. Using .gitignore file for better filtering files you want to count
2. Hidden files aka files starting with . are ignored by default
3. Removing all comments/documentation
4. Deleting all "unnecessary" \n
