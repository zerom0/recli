# reMarkable CLI

The reMarkable CLI (`recli`) is a commandline tool that allows you to browse, 
manage and sync files on your reMarkable device without any internet 
connectivity.
Just connect your reMarkable to your PC with an USB cable, enable the USB web
interface on the device and you are ready to go.

## Commands

### List files on reMarkable
`recli list [-r] [<path>]`

### Retrieve a file from the reMarkable
`recli get <path>`

### Synchronize rendered files from reMarkable
`recli sync <path>`

### Store a file on the reMarkable (todo)
`recli add <file> <path>`

### Delete a file from the reMarkable (todo)
`recli remove [-f] <path>`

### Backup the reMarkable content to an archive (todo)
`recli backup [<file>]`

### Restore the reMarkable content from an archive (todo)
`recli restore [-f] <file>`

### Erase the reMarkable content (todo)
`recli erase [-f]`

### Install a template (todo)
`recli template add [-f] <file>`

### Remove a template (doto)
`recli template remove [-f] <path>`