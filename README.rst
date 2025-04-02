========================
Ryst
========================

Ryst (Rsync Tool) is a tool that allows granular *rsync* copies.

Why This Tool Exists
======================
This tools is a convenience tool that allows you to have easy, predefined, rysnc
sources and targets. It eliminates the need to write specialty scripts and
instead places all the details of what and where to copy in a single file:
*.ryst*. Of course we support exclude files set by the *.exclude-from* variable.

If you have multiple sources and destinations, this tool makes quick work of
specifying those quickly and efficiently, without the need to write specialty
scripts.

Features
=========
We support a small subset of *rsync* features:

* A simple configuration file called *.ryst* in the calling folder.
* Supports .exclude files supported by *rsync*
* Supports multiple source folders
* Supports multiple dest folders
* Supports *rsync* boolean flags (specified by yes or true):

  - compress
  - cvs-exclude
  - delete
  - dry-run
  - verbose

* Supports select *rsync* variables (in key=value format):

  - bwlimit
  - exclude
  - exclude-from
  - include
  - include-from
  - source

Installation
===============
Installation is easy:

* Install rust:: 

    curl https://sh.rustup.rs -sSf | sh

* Copy this repo to /tmp/::

    git clone https://github.com/fortuitous-dev/ryst

* Ensure *make* is installed on your Linux system. This is usually the case, but
  if not, use your package manager to install it.

* Run the install command::

    make install

Uninstallation
================
Just remove the tool from the cargo folder like this::

   rm $(which ryst)

Configuration
=======================

Sources Destinations
----------------------

Ryst supports multiple source and destination (dest) targets.
This give you granular control over source and destination targets.
The configuration file for this can look like::

   #-------------------------------------------------------
   # This is the .ryst configuration file for Rsync Tool (rst)
   #-------------------------------------------------------
   # source file. can be ./ or server:path/to/here/
   # dest can be user@host:. or a folder or just dot (.)
   source=./acme
   source=./tools
   dest=/data/
   dest=joy@acme.org:backups/
   # exclude-from can be any rsync pattern file
   exclude-from=.exclude
   # Excludes can be include style too
   include=.include
   # Other options
   delete=yes
   compress=no
   dry-run=no
   verbose=yes
   exclude-csv=true

This file will copy both *./acme* and *./tools* folders (relative to the .ryst
folder location) over to both */data/* and the remote *joy@acme.org:backups/* .
In addition, all non-source files will be deleted on the destinations.

Use
------
Once you create your *.ryst* file in a folder, navigate to that folder and
execute::

   ryst

.. Note::

   You are strongly encouraged to set **dry-run=yes** when setting up the tool
   so that you can avoid early errors. Once the copies look correct, go back and 
   set **dry-run=no** and try it out.

Construction
==============
Ryst is written in Rust. Instead of trying to recreate all the features of rsync
natively in Rust, we simply wrap the *rsync* tool with the Rust
**std::process::Command** tools. Given the stability and features in *rsync*,
this makes **ryst** very flavorful and robust.

Potential Improvements
=========================
Among the many improvements that could be made, these seem to be prominent:

* Pre and Post copy commands, to shutdown databases or services that require
  syncronization of files before copy.
* Better tests
* Support more rsync features
