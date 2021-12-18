# lxsdk bindings

This crate is ffi bindings for Modo's LXSDK COM Interface. 

To build this yourself, download the LXSDK from The Foundry.

Copy the headers from LXSDK######/include/ => lxsdk/include/

To recreate the bindings.h I also added a python script to scan this folder and #include "{header}" for each header in the include folder as I didn't want to explore doing this in rust just yet.