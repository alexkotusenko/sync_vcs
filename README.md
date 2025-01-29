# Synchronisation version control system

Useful when dealing with annoying sync clashes with cloud storage. Imagine you're working on a file and changing it every couple minutes or so. These changes may not always register properly, especially for pdf files (at least in my experience). I created this program to "commit" all of your changes at once. 

The program needs a path to a csv wiring file as input.

Every single file you want to sync needs to be accounted for (at least in this version)

The filename of in the local and cloud directory is the same. So a file called ```letter.pdf``` will be called ```letter.pdf``` in the cloud folder too.

### CSV file expected fields
The following fields are expected in this very order
1. `filename` - name of the FILE being synced
2. `local_directory`- name of the folder containing the synced file
  - if this directory does not exist, the problematic file will be skipped without panicking
3. `sync_directory` - name of the folder the file will be synced to
  - if this directory does not exist, the problematic file will be skipped without panicking


### CSV file structure & example row
(Once again, make sure the fields are specified in the right order)
```
filename,local_directory,sync_directory,overwrite_allowed
letter.pdf,/home/username/Documents,/home/username/sync/Nextcloud/Documents,y
```
possible values for overwrite_allowed:
`y`, `Y`, `n`, `N` (no "true" or "false")

### Usage

```./binary_name <CSV_FILEPATH>```

Any other argument sequence is invalid

### Considerations
- If some row is unparsable, it will be skipped
- If some row has an unexpected amount of fields, the app will panic
- If you want to disable overwriting if a file with the same exists in the sync directory, disable it with `n`/`N` in the wiring file

You have granular control over how every file is synced. 
