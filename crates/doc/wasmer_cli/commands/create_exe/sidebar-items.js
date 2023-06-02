window.SIDEBAR_ITEMS = {"constant":[["LINK_SYSTEM_LIBRARIES_UNIX",""],["LINK_SYSTEM_LIBRARIES_WINDOWS",""]],"enum":[["AllowMultiWasm","In pirita mode, specifies whether multi-atom pirita files should be allowed or rejected"],["UrlOrVersion","Url or version to download the release from"]],"fn":[["compile_atoms",""],["compile_pirita_into_directory","Given a pirita file, compiles the .wasm files into the target directory"],["create_header_files_in_dir","Create the static_defs.h header files in the /include directory"],["generate_wasmer_main_c","Generate the wasmer_main.c that links all object files together (depending on the object format / atoms number)"],["get_entrypoint",""],["get_module_infos",""],["link_exe_from_dir","Given a directory, links all the objects from the directory appropriately"],["link_objects_system_linker","Link compiled objects using the system linker"],["prepare_directory_from_single_wasm_file","Given a .wasm file, compiles the .wasm file into the target directory and creates the entrypoint.json"],["run_c_compile","Compile the C code."],["serialize_volume_to_webc_v1",""],["volume_file_block","Serialize a set of volumes so they can be read by the C API."],["write_entrypoint",""],["write_volume_obj",""]],"mod":[["http_fetch",""],["utils",""]],"struct":[["CommandEntrypoint","Command entrypoint for multiple commands"],["CreateExe","The options for the `wasmer create-exe` subcommand"],["CrossCompile",""],["CrossCompileSetup",""],["Entrypoint","Given a pirita file, determines whether the file has one default command as an entrypoint or multiple (need to be specified via –command)"],["PrefixMapCompilation","Prefix map used during compilation of object files"],["Volume","Volume object file (name + path to object file)"]]};