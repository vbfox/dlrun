# DlRun

A small command line tool to download files and installers in windows docker build files.
It support both .exe and .msi installers.

## Why does it exists

When creating windows docker containers downloading installers, verifying their hash, running them and then removing
them is a basic task often repeated. This tool is there to automate theses tasks.

## Sample usage

```dockerfile
FROM microsoft/windowsservercore:1803

COPY dlrun.exe .
RUN dlrun run https://nodejs.org/dist/v10.5.0/node-v10.5.0-x64.msi BAEE3A34ECBE9040C6DA8F01EB61BDE563A0458A94401A80DD87229FC938ADD4
```

## Command line

* `dlrun download URL [HASH]` Download the file
* `dlrun run URL [HASH]` Download, run then delete the file
  * `--keep` Don't delete the file afterwards
  * `--` Used to separate dlrun arguments from the arguments that will be passed to the installer
* Common arguments:
  * `--file-name` Name of the file (The last part of the URL is used by default)
  * `--folder` Folder where the file should be saved
