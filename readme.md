# reMarkable CLI

The reMarkable CLI (`recli`) is a commandline interface that allows you to browse, 
manage and sync files on your reMarkable device without any internet 
connectivity.
Just connect your reMarkable to your PC with an USB cable, enable the USB web
interface on the device and you are ready to go.

## Commands

### Check connection to reMarkable
`recli status`

### List files on reMarkable
`recli list [-r] [<path>]`

### Synchronize rendered files from reMarkable
`recli sync <path>`

### Store a file on the reMarkable
`recli add <file> <path>`

### Retrieve a file from the reMarkable
`recli get <path>`

### Delete a file from the reMarkable
`recli remove [-f] <path>`

### Backup the reMarkable content to an archive
`recli backup [<file>]`

### Erase the reMarkable content
`recli erase [-f]`

### Restore the reMarkable content from an archive
`recli restore [-f] <file>`

### Install a template
`recli template add [-f] <file>`

### Remove a template
`recli template remove [-f] <path>`