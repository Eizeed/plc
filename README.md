Project line counter
cli-tool to count lines in your projects

Roadmap:
1) Make better gitignore algo to use several gitignore files for certain folders
2) Make verbose flag to show all recursive directory visits
3) Make debug flag to show all outputs through programm

How to use:
pcl [PATH] [FILE EXTENSION]

features:
Using .gitignore file for better filtering files you want to count
Hidden files aka files starting with . are ignored by default
Trimming file as string
Deleting all "unnecessary" new lines aka '\n'
