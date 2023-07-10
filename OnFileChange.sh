#!/bin/bash
# Usage: 
# OnFileChange.sh PathToWatch IncludePattern Command
# PathToWatch   : Monitor this path for file changes. The Command will be executed each time a file 
#                 gets modified, moved, created or deleted within this path and all its subdirectories
# IncludePattern: Only monitor files that match this pattern
# Command       : This command will be executed each time a file in the monitored path (that matches the IncludePattern)
#                 is modified, created, moved or deleted

# install the inotifywait command if not already present
 sudo apt -y install inotify-tools
 
 # sanity check whether the package was installed correctly
if [ -z "$(which inotifywait)" ]; then
    echo "inotifywait not installed."
    echo "In most distros, it is available in the inotify-tools package."
    exit 1
fi
 
counter=0;
 
 # execute the command that was provided to the script
function execute() {
    counter=$((counter+1))
    if [ $((counter%2)) -eq 1 ]; then
        echo "########## Detected change ##########" 
        eval $@
    fi
}
 
# start the monitoring with the configuration provided by the parameters
inotifywait \
    --recursive \
    --monitor \
    --format "%e %w%f" \
    --include $2 \
    --event modify,move,create,delete \
    $1 \
| while read changed; do
    # a change was detected, now execute the command
    execute $3
done