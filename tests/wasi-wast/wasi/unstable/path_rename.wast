(wasi_test "path_rename.wasm"
  (temp_dirs "temp")
  (assert_return (i64.const 0))
  (assert_stdout "The original file does not still exist!\nFound item: path_renamed_file.txt\n柴犬\nrun_with_sub_dir: The original file does not still exist!\nrun_with_different_sub_dirs: The original file does not still exist!\n")
)