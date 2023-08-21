# bats file_tags=tag:this
skip
@test "this/this_at_top_level.lox" {
  run target/debug/lox test/cases/this/this_at_top_level.lox

}
