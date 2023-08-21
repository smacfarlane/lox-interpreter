# bats file_tags=tag:this
skip
@test "this/this_in_top_level_function.lox" {
  run target/debug/lox test/cases/this/this_in_top_level_function.lox

}
